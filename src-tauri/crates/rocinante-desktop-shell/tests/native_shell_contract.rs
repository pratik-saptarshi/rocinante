use rocinante_desktop_shell::{
    deep_link::{parse_deep_link, DeepLinkTarget},
    navigation_shortcut, window_close_behavior, NavigationAction, ShellLifecycle, ShellPage,
    ShellState, ShortcutKey, WindowCloseBehavior, WindowProfile,
};
use std::collections::HashMap;
use std::path::PathBuf;

#[cfg(feature = "native-ui")]
#[derive(Default)]
struct MemoryStorage(HashMap<String, String>);

#[cfg(feature = "native-ui")]
impl eframe::Storage for MemoryStorage {
    fn get_string(&self, key: &str) -> Option<String> {
        self.0.get(key).cloned()
    }

    fn set_string(&mut self, key: &str, value: String) {
        self.0.insert(key.to_owned(), value);
    }

    fn remove_string(&mut self, key: &str) {
        self.0.remove(key);
    }

    fn flush(&mut self) {}
}

#[test]
fn native_shell_navigation_is_explicit_and_preserves_route_state() {
    let mut shell = ShellState::default();
    shell.dispatch(NavigationAction::OpenRepositories);
    assert_eq!(shell.page(), ShellPage::Repositories);

    shell.dispatch(NavigationAction::OpenAlerts);
    assert_eq!(shell.page(), ShellPage::Alerts);
}

#[test]
fn native_shell_lifecycle_accepts_start_then_close_only() {
    let mut shell = ShellState::default();
    assert_eq!(shell.lifecycle(), ShellLifecycle::Ready);
    shell.dispatch(NavigationAction::Close);
    assert_eq!(shell.lifecycle(), ShellLifecycle::Closed);
    shell.dispatch(NavigationAction::OpenDashboard);
    assert_eq!(shell.lifecycle(), ShellLifecycle::Closed);
}

#[test]
fn native_shell_close_hides_with_tray_but_explicit_quit_exits() {
    assert_eq!(
        window_close_behavior(true, false),
        WindowCloseBehavior::Hide
    );
    assert_eq!(window_close_behavior(true, true), WindowCloseBehavior::Exit);
    assert_eq!(
        window_close_behavior(false, false),
        WindowCloseBehavior::Exit
    );
}

#[test]
fn native_window_profile_preserves_desktop_baseline_dimensions() {
    let profile = WindowProfile::default();
    assert_eq!(profile.title, "Rocinante Repo Analyzer");
    assert_eq!((profile.width, profile.height), (1200, 800));
    assert!(profile.resizable);
}

#[test]
fn native_shell_route_state_roundtrips_for_startup_restore() {
    let mut shell = ShellState::default();
    shell.dispatch(NavigationAction::OpenSettings);

    let saved = serde_json::to_string(&shell).expect("serialize shell state");
    let restored: ShellState = serde_json::from_str(&saved).expect("restore shell state");
    assert_eq!(restored.page(), ShellPage::Settings);
    assert_eq!(restored.lifecycle(), ShellLifecycle::Ready);
}

#[test]
fn repository_selection_is_saved_and_restored_with_shell_state() {
    let mut shell = ShellState::default();
    shell.select_repository(Some(PathBuf::from("/work/project")));

    let saved = serde_json::to_string(&shell).expect("serialize shell state");
    let restored: ShellState = serde_json::from_str(&saved).expect("restore shell state");
    assert_eq!(
        restored.selected_repository(),
        Some(PathBuf::from("/work/project"))
    );
}

#[test]
#[cfg(feature = "native-ui")]
fn repository_selection_roundtrips_through_eframe_ron_storage() {
    let mut shell = ShellState::default();
    shell.select_repository(Some(PathBuf::from("/work/project with spaces")));

    let mut storage = MemoryStorage::default();
    eframe::set_value(&mut storage, "rocinante_shell_state", &shell);
    let restored: ShellState = eframe::get_value(&storage, "rocinante_shell_state")
        .expect("restore shell state from eframe RON storage");

    assert_eq!(restored.page(), shell.page());
    assert_eq!(restored.selected_repository(), shell.selected_repository());
}

#[test]
fn repository_selection_roundtrips_through_json_storage_key() {
    let mut shell = ShellState::default();
    shell.select_repository(Some(PathBuf::from("/work/project with spaces")));

    let mut storage = HashMap::new();
    let saved = serde_json::to_string(&shell).expect("serialize shell state as JSON");
    storage.insert("rocinante_shell_state_json", saved);
    let restored: ShellState = serde_json::from_str(
        &storage
            .get("rocinante_shell_state_json")
            .expect("load shell state JSON"),
    )
    .expect("restore shell state from JSON storage key");

    assert_eq!(restored.page(), shell.page());
    assert_eq!(restored.selected_repository(), shell.selected_repository());
}

#[cfg(unix)]
#[test]
fn repository_selection_preserves_non_utf8_unix_path_bytes() {
    use std::os::unix::ffi::{OsStrExt, OsStringExt};

    let original = PathBuf::from(std::ffi::OsString::from_vec(vec![b'/', b'w', b'/', 0xff]));
    let mut shell = ShellState::default();
    shell.select_repository(Some(original.clone()));

    let saved = serde_json::to_string(&shell).expect("serialize shell state");
    let restored: ShellState = serde_json::from_str(&saved).expect("restore shell state");
    assert_eq!(
        restored
            .selected_repository()
            .expect("repository path")
            .as_os_str()
            .as_bytes(),
        original.as_os_str().as_bytes()
    );
}

#[test]
fn legacy_utf8_repository_path_restores_from_prior_shell_state() {
    let restored: ShellState = serde_json::from_str(
        r#"{"page":"Repositories","selected_repository":"/work/old-project"}"#,
    )
    .expect("restore legacy shell state");
    assert_eq!(
        restored.selected_repository(),
        Some(PathBuf::from("/work/old-project"))
    );
}

#[test]
fn command_shortcuts_map_routes_and_require_command_modifier() {
    assert_eq!(
        navigation_shortcut(true, ShortcutKey::Digit1),
        Some(NavigationAction::OpenDashboard)
    );
    assert_eq!(
        navigation_shortcut(true, ShortcutKey::Digit2),
        Some(NavigationAction::OpenRepositories)
    );
    assert_eq!(
        navigation_shortcut(true, ShortcutKey::Digit3),
        Some(NavigationAction::OpenAlerts)
    );
    assert_eq!(
        navigation_shortcut(true, ShortcutKey::Digit4),
        Some(NavigationAction::OpenSettings)
    );
    assert_eq!(navigation_shortcut(false, ShortcutKey::Digit1), None);
    assert_eq!(
        navigation_shortcut(true, ShortcutKey::Q),
        Some(NavigationAction::Close)
    );
    assert_eq!(
        navigation_shortcut(true, ShortcutKey::O),
        Some(NavigationAction::ChooseRepository)
    );
}

#[test]
fn repository_deep_link_restores_the_selected_path_and_repository_route() {
    let target = parse_deep_link("rocinante://repository/open?path=%2Ftmp%2Fsample%20repo")
        .expect("valid repository deep link");
    let DeepLinkTarget::OpenRepository(path) = target;
    assert_eq!(path, PathBuf::from("/tmp/sample repo"));

    let mut state = ShellState::default();
    state.apply_deep_link(DeepLinkTarget::OpenRepository(path.clone()));
    assert_eq!(state.page(), ShellPage::Repositories);
    assert_eq!(state.selected_repository(), Some(path));
}

#[test]
fn repository_deep_links_reject_unknown_schemes_and_relative_paths() {
    assert!(parse_deep_link("https://repository/open?path=%2Ftmp%2Frepo").is_none());
    assert!(parse_deep_link("rocinante://repository/open?path=relative").is_none());
    assert!(
        parse_deep_link("rocinante://repository/open?path=%2Ftmp%2Fa&path=%2Ftmp%2Fb").is_none()
    );
    assert!(parse_deep_link("rocinante://repository/open?path=%2Ftmp%2Frepo&mode=edit").is_none());
    assert!(parse_deep_link("rocinante://settings").is_none());
}

#[cfg(unix)]
#[test]
fn repository_deep_link_preserves_non_utf8_path_bytes() {
    use std::os::unix::ffi::OsStrExt;

    let target = parse_deep_link("rocinante://repository/open?path=%2Ftmp%2F%FF")
        .expect("valid repository deep link");
    let DeepLinkTarget::OpenRepository(path) = target;
    assert_eq!(path.as_os_str().as_bytes(), b"/tmp/\xff");
}
