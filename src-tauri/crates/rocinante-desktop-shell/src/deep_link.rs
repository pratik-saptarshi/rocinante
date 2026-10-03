use std::path::PathBuf;
#[cfg(feature = "native-ui")]
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

#[cfg(feature = "native-ui")]
pub const MAX_DEEP_LINK_BYTES: usize = 8 * 1024;
#[cfg(feature = "native-ui")]
pub const MAX_PENDING_DEEP_LINKS: usize = 64;
#[cfg(feature = "native-ui")]
const ACTIVATE_MESSAGE: &[u8] = b"rocinante:activate:v1";
#[cfg(feature = "native-ui")]
static NEXT_MESSAGE_ID: AtomicU64 = AtomicU64::new(0);

#[cfg(feature = "native-ui")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeepLinkEvent {
    Activate,
    OpenRepository(PathBuf),
}

#[cfg(feature = "native-ui")]
pub struct DeepLinkInbox {
    directory: PathBuf,
    lock_file: File,
    primary: bool,
}

#[cfg(feature = "native-ui")]
impl DeepLinkInbox {
    pub fn open(app_data_dir: &Path) -> io::Result<Self> {
        let directory = app_data_dir.join("deep-link-inbox");
        fs::create_dir_all(&directory)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&directory, fs::Permissions::from_mode(0o700))?;
        }

        let lock_file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(directory.join("instance.lock"))?;
        let primary = match fs2::FileExt::try_lock_exclusive(&lock_file) {
            Ok(()) => true,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => false,
            Err(error) => return Err(error),
        };
        Ok(Self {
            directory,
            lock_file,
            primary,
        })
    }

    pub fn is_primary(&self) -> bool {
        self.primary
    }

    pub fn forward(&self, uri: &str) -> io::Result<()> {
        if self.primary {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "the primary instance cannot forward a link to itself",
            ));
        }
        if uri.len() > MAX_DEEP_LINK_BYTES || parse_deep_link(uri).is_none() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "deep link is invalid or exceeds the size limit",
            ));
        }
        self.enqueue(uri.as_bytes())
    }

    pub fn activate(&self) -> io::Result<()> {
        if self.primary {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "the primary instance cannot activate itself through the inbox",
            ));
        }
        self.enqueue(ACTIVATE_MESSAGE)
    }

    pub fn drain(&self) -> Vec<DeepLinkEvent> {
        if !self.primary {
            return Vec::new();
        }
        let Ok(entries) = fs::read_dir(&self.directory) else {
            return Vec::new();
        };
        let mut files = entries
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "pending"))
            .collect::<Vec<_>>();
        files.sort_by_key(|entry| pending_sequence(&entry.file_name()));

        files
            .into_iter()
            .filter_map(|entry| {
                if !entry.file_type().ok()?.is_file() {
                    let _ = fs::remove_file(entry.path());
                    return None;
                }
                let file = File::open(entry.path()).ok()?;
                let mut bytes = Vec::new();
                let _ = file
                    .take((MAX_DEEP_LINK_BYTES + 1) as u64)
                    .read_to_end(&mut bytes);
                let _ = fs::remove_file(entry.path());
                if bytes.len() > MAX_DEEP_LINK_BYTES {
                    return None;
                }
                if bytes == ACTIVATE_MESSAGE {
                    return Some(DeepLinkEvent::Activate);
                }
                let uri = std::str::from_utf8(&bytes).ok()?;
                match parse_deep_link(uri)? {
                    DeepLinkTarget::OpenRepository(path) => {
                        Some(DeepLinkEvent::OpenRepository(path))
                    }
                }
            })
            .collect()
    }

    fn enqueue(&self, message: &[u8]) -> io::Result<()> {
        let queue_lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(self.directory.join("queue.lock"))?;
        fs2::FileExt::lock_exclusive(&queue_lock)?;
        self.cleanup_temporary_files()?;
        let pending_count = fs::read_dir(&self.directory)?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "pending"))
            .count();
        if pending_count >= MAX_PENDING_DEEP_LINKS {
            return Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "deep-link inbox is full",
            ));
        }
        let sequence = fs::read_dir(&self.directory)?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "pending"))
            .filter_map(|entry| parse_pending_sequence(&entry.file_name()))
            .max()
            .unwrap_or(0)
            .checked_add(1)
            .ok_or_else(|| io::Error::other("deep-link sequence exhausted"))?;
        let id = NEXT_MESSAGE_ID.fetch_add(1, Ordering::Relaxed);
        let stem = format!("request-{}-{sequence:020}-{id}", std::process::id());
        let temporary = self.directory.join(format!(".{stem}.tmp"));
        let pending = self.directory.join(format!("{stem}.pending"));
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        let result = (|| {
            file.write_all(message)?;
            file.sync_all()?;
            fs::rename(&temporary, &pending)
        })();
        if result.is_err() {
            let _ = fs::remove_file(temporary);
        }
        result
    }

    fn cleanup_temporary_files(&self) -> io::Result<()> {
        for entry in fs::read_dir(&self.directory)? {
            let entry = entry?;
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with(".request-")
                && name.ends_with(".tmp")
                && entry.file_type()?.is_file()
            {
                fs::remove_file(entry.path())?;
            }
        }
        Ok(())
    }
}

#[cfg(feature = "native-ui")]
fn pending_sequence(file_name: &std::ffi::OsStr) -> u64 {
    parse_pending_sequence(file_name).unwrap_or(u64::MAX)
}

#[cfg(feature = "native-ui")]
fn parse_pending_sequence(file_name: &std::ffi::OsStr) -> Option<u64> {
    file_name.to_str().and_then(|name| {
        let mut pieces = name.split('-');
        (pieces.next()? == "request").then_some(())?;
        pieces.next()?;
        pieces.next()?.parse().ok()
    })
}

#[cfg(feature = "native-ui")]
impl Drop for DeepLinkInbox {
    fn drop(&mut self) {
        if self.primary {
            let _ = fs2::FileExt::unlock(&self.lock_file);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeepLinkTarget {
    OpenRepository(PathBuf),
}

pub fn parse_deep_link(value: &str) -> Option<DeepLinkTarget> {
    let url = url::Url::parse(value).ok()?;
    if url.scheme() != "rocinante" || url.host_str() != Some("repository") || url.path() != "/open"
    {
        return None;
    }

    let query = url.query()?;
    let (key, value) = query.split_once('=')?;
    if key != "path" || value.contains('&') {
        return None;
    }
    let path = path_from_query_value(value)?;
    path.is_absolute()
        .then_some(DeepLinkTarget::OpenRepository(path))
}

fn path_from_query_value(value: &str) -> Option<PathBuf> {
    let encoded = value.as_bytes();
    let mut decoded = Vec::with_capacity(encoded.len());
    let mut index = 0;
    while index < encoded.len() {
        match encoded[index] {
            b'+' => decoded.push(b' '),
            b'%' => {
                let high = hex_value(*encoded.get(index + 1)?)?;
                let low = hex_value(*encoded.get(index + 2)?)?;
                decoded.push((high << 4) | low);
                index += 2;
            }
            byte => decoded.push(byte),
        }
        index += 1;
    }

    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;
        Some(PathBuf::from(std::ffi::OsString::from_vec(decoded)))
    }
    #[cfg(not(unix))]
    {
        Some(PathBuf::from(String::from_utf8(decoded).ok()?))
    }
}

fn hex_value(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}
