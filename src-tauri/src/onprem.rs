use crate::errors::AnalyzerError;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

pub trait InternalGitProvider: Send + Sync {
    fn repo_url(&self, repo_name: &str) -> Result<String, AnalyzerError>;
}

pub trait DirectoryProvider: Send + Sync {
    fn is_in_group(&self, user: &str, group: &str) -> Result<bool, AnalyzerError>;
}

pub trait DirectoryLookup: Send + Sync {
    fn groups_for_user(&self, user: &str) -> Result<Vec<String>, AnalyzerError>;
}

pub struct LocalGitProvider;
impl InternalGitProvider for LocalGitProvider {
    fn repo_url(&self, repo_name: &str) -> Result<String, AnalyzerError> {
        Ok(format!("ssh://git.internal/{}.git", repo_name))
    }
}

pub struct StaticDirectoryLookup {
    memberships: HashMap<String, Vec<String>>,
    lookups: AtomicUsize,
}

impl StaticDirectoryLookup {
    pub fn new(memberships: HashMap<String, Vec<String>>) -> Self {
        let memberships = memberships
            .into_iter()
            .map(|(user, groups)| {
                (
                    normalize_principal(&user, "user").unwrap_or_else(|_| user),
                    groups
                        .into_iter()
                        .map(|group| normalize_principal(&group, "group").unwrap_or_else(|_| group))
                        .collect(),
                )
            })
            .collect();
        Self {
            memberships,
            lookups: AtomicUsize::new(0),
        }
    }

    pub fn lookup_count(&self) -> usize {
        self.lookups.load(Ordering::SeqCst)
    }
}

impl DirectoryLookup for StaticDirectoryLookup {
    fn groups_for_user(&self, user: &str) -> Result<Vec<String>, AnalyzerError> {
        self.lookups.fetch_add(1, Ordering::SeqCst);
        Ok(self
            .memberships
            .get(&normalize_principal(user, "user")?)
            .cloned()
            .unwrap_or_default())
    }
}

pub struct ActiveDirectoryProvider {
    source: Arc<dyn DirectoryLookup>,
    group_aliases: HashMap<String, String>,
    membership_cache: Mutex<HashMap<(String, String), bool>>,
}

impl ActiveDirectoryProvider {
    pub fn new(source: Arc<dyn DirectoryLookup>) -> Self {
        Self::with_group_aliases(source, HashMap::new())
    }

    pub fn with_group_aliases(
        source: Arc<dyn DirectoryLookup>,
        group_aliases: HashMap<String, String>,
    ) -> Self {
        let group_aliases = group_aliases
            .into_iter()
            .filter_map(|(alias, canonical)| {
                let alias = normalize_principal(&alias, "group alias").ok()?;
                let canonical = normalize_principal(&canonical, "group alias target").ok()?;
                Some((alias, canonical))
            })
            .collect();

        Self {
            source,
            group_aliases,
            membership_cache: Mutex::new(HashMap::new()),
        }
    }

    pub fn cache_size(&self) -> usize {
        self.membership_cache
            .lock()
            .map(|cache| cache.len())
            .unwrap_or_default()
    }

    fn canonical_group(&self, group: &str) -> Result<String, AnalyzerError> {
        let normalized = normalize_principal(group, "group")?;
        let mut current = normalized;
        let mut visited = HashSet::new();

        for _ in 0..8 {
            if !visited.insert(current.clone()) {
                return Err(AnalyzerError::PermissionDenied(format!(
                    "group alias cycle detected for {group}"
                )));
            }

            match self.group_aliases.get(&current) {
                Some(next) if next != &current => current = next.clone(),
                _ => return Ok(current),
            }
        }

        Err(AnalyzerError::PermissionDenied(format!(
            "group alias chain too deep for {group}"
        )))
    }
}

impl DirectoryProvider for ActiveDirectoryProvider {
    fn is_in_group(&self, user: &str, group: &str) -> Result<bool, AnalyzerError> {
        let user = normalize_principal(user, "user")?;
        let group = self.canonical_group(group)?;

        if let Some(cached) = self
            .membership_cache
            .lock()
            .map_err(|e| AnalyzerError::PermissionDenied(e.to_string()))?
            .get(&(user.clone(), group.clone()))
            .copied()
        {
            return Ok(cached);
        }

        let groups = self.source.groups_for_user(&user)?;
        let allowed = groups
            .into_iter()
            .map(|group_name| normalize_principal(&group_name, "group"))
            .collect::<Result<HashSet<_>, _>>()?
            .contains(&group);

        self.membership_cache
            .lock()
            .map_err(|e| AnalyzerError::PermissionDenied(e.to_string()))?
            .insert((user, group), allowed);

        Ok(allowed)
    }
}

fn normalize_principal(value: &str, field: &str) -> Result<String, AnalyzerError> {
    let normalized = value.trim();
    if normalized.is_empty() {
        return Err(AnalyzerError::PermissionDenied(format!("blank {field}")));
    }
    if normalized.chars().any(|c| c.is_control()) {
        return Err(AnalyzerError::PermissionDenied(format!(
            "invalid {field} contains control characters"
        )));
    }
    Ok(normalized.to_lowercase())
}
