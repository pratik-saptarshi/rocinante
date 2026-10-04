#![cfg(feature = "native-ui")]

use rocinante_desktop_shell::deep_link::{
    DeepLinkEvent, DeepLinkInbox, MAX_DEEP_LINK_BYTES, MAX_PENDING_DEEP_LINKS,
};
use std::process::Command;

#[test]
fn linux_desktop_entry_registers_and_forwards_rocinante_links() {
    let desktop_entry = include_str!("../packaging/linux/rocinante.desktop");

    assert!(desktop_entry.contains("MimeType=x-scheme-handler/rocinante;"));
    assert!(desktop_entry
        .lines()
        .any(|line| line == "Exec=rocinante-desktop-shell %u"));
}

#[test]
fn linux_url_acceptance_exercises_registered_cold_warm_and_restart_flow() {
    let acceptance = include_str!("../../../../scripts/test-linux-url-dispatch.sh");
    let shell_source = include_str!("../src/lib.rs");

    for required in [
        "packaging/linux/install-user.sh",
        "ROCINANTE_ACCEPTANCE_NOTIFICATION=1",
        "dunst --config \"$dunst_config\" --print",
        "dunstctl count displayed",
        "wait_for_notification",
        "Rocinante notification acceptance",
        "xdg-mime query default x-scheme-handler/rocinante",
        "gio open \"$(make_uri \"$cold_path\")\"",
        "gio open \"$(make_uri \"$warm_path\")\"",
        "if [[ \"$(sed -n 's/^pid=//p' \"$witness\")\" != \"$test_pid\" ]]",
        "touch \"$quit_file\"",
        "rm -f \"$quit_file\"",
        "The app did not persist eframe state before exit",
        "\"$installed_binary\" >/dev/null 2>&1 &",
        "Linux registered cold/warm URI delivery and saved-state restart passed",
    ] {
        assert!(
            acceptance.contains(required),
            "missing Linux acceptance step: {required}"
        );
    }
    assert!(
        acceptance.find("rm -f \"$quit_file\"").unwrap()
            < acceptance
                .find(": > \"$witness\"\n\"$installed_binary\" >/dev/null 2>&1 &")
                .unwrap(),
        "the restart must not inherit the previous process's clean-quit request"
    );
    assert!(shell_source.contains("#[cfg(feature = \"acceptance-witness\")]\n            notify_acceptance_surface_if_requested();"));
    assert!(shell_source.contains("fn notify_acceptance_surface_if_requested()"));
    assert!(shell_source.contains(
        "send_desktop_notification(\n                \"Rocinante notification acceptance\""
    ));
}

#[cfg(unix)]
#[test]
fn linux_installer_registers_binary_and_uri_handler_in_user_scope() {
    use std::{os::unix::fs::PermissionsExt, path::PathBuf};

    let temp = tempfile::tempdir().expect("temporary install root");
    let home = temp.path().join("home with \"quote $space %u \\back");
    let data = temp.path().join("data");
    let tools = temp.path().join("tools");
    let tool_calls = temp.path().join("tool-calls");
    let duckdb_library = temp.path().join("deps/libduckdb.so");
    std::fs::create_dir_all(&home).expect("create temporary home");
    std::fs::create_dir_all(&tools).expect("create command shims");
    std::fs::create_dir_all(duckdb_library.parent().unwrap()).expect("create library fixture");
    std::fs::write(&duckdb_library, b"verified prebuilt DuckDB fixture")
        .expect("write DuckDB library fixture");
    let patchelf = tools.join("patchelf");
    std::fs::write(
        &patchelf,
        "#!/bin/sh\nprintf '%s\\n' \"$*\" >> \"$ROCINANTE_INSTALLER_CALL_LOG\"\n",
    )
    .expect("write patchelf command shim");
    std::fs::set_permissions(&patchelf, std::fs::Permissions::from_mode(0o755))
        .expect("make patchelf shim executable");
    for command in ["xdg-mime", "update-desktop-database"] {
        let path = tools.join(command);
        std::fs::write(
            &path,
            "#!/bin/sh\nprintf '%s %s\\n' \"$0\" \"$*\" >> \"$ROCINANTE_INSTALLER_CALL_LOG\"\n",
        )
        .expect("write command shim");
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
            .expect("make command shim executable");
    }
    let binary = temp.path().join("deps/../source-binary");
    std::fs::write(&binary, "#!/bin/sh\nexit 0\n").expect("write source binary");
    std::fs::set_permissions(&binary, std::fs::Permissions::from_mode(0o755))
        .expect("make source binary executable");
    let mut path_entries = vec![tools];
    path_entries.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    let path = std::env::join_paths(path_entries).expect("construct command path");
    let installer =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("packaging/linux/install-user.sh");

    let output = Command::new("sh")
        .arg(installer)
        .arg(&binary)
        .env("HOME", &home)
        .env("XDG_DATA_HOME", &data)
        .env("ROCINANTE_INSTALLER_CALL_LOG", &tool_calls)
        .env("PATH", path)
        .output()
        .expect("run Linux user installer");
    assert!(
        output.status.success(),
        "installer failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(home.join(".local/bin/rocinante-desktop-shell").is_file());
    assert_eq!(
        std::fs::read(home.join(".local/lib/rocinante/libduckdb.so")).unwrap(),
        b"verified prebuilt DuckDB fixture"
    );
    assert!(data
        .join("icons/hicolor/512x512/apps/rocinante.png")
        .is_file());
    let desktop_entry = std::fs::read_to_string(data.join("applications/rocinante.desktop"))
        .expect("installed desktop entry");
    assert!(desktop_entry.contains("MimeType=x-scheme-handler/rocinante;"));
    assert!(desktop_entry.contains("rocinante-desktop-shell\" %u"));
    assert!(desktop_entry.contains("home with \\\"quote \\\\$space"));
    assert!(desktop_entry.contains("%%u"));
    assert!(desktop_entry.contains("\\\\\\\\back/.local"));
    assert!(
        desktop_entry.lines().any(|line| {
            line.starts_with("TryExec=")
                && !line.starts_with("TryExec=\"")
                && line.contains("/.local/bin/rocinante-desktop-shell")
                && line.contains("home\\swith\\s\"quote\\s$space\\s%u")
                && line.contains("%u\\s")
                && line.contains("\\\\back/.local")
        }),
        "unexpected installed desktop entry:\n{desktop_entry}"
    );
    assert!(!desktop_entry.contains("TryExec=rocinante-desktop-shell"));
    let tool_calls = std::fs::read_to_string(tool_calls).expect("read registration tool calls");
    assert!(tool_calls.contains("update-desktop-database "));
    assert!(tool_calls.contains("xdg-mime default rocinante.desktop x-scheme-handler/rocinante"));
    assert!(tool_calls.contains("--set-rpath $ORIGIN/../lib/rocinante"));
}

#[test]
fn secondary_instance_forwards_valid_deep_link_to_primary() {
    let data_dir = tempfile::tempdir().expect("temporary app data directory");
    let primary = DeepLinkInbox::open(data_dir.path()).expect("acquire primary instance");
    let secondary = DeepLinkInbox::open(data_dir.path()).expect("open secondary instance");

    assert!(primary.is_primary());
    assert!(!secondary.is_primary());
    secondary
        .forward("rocinante://repository/open?path=%2Ftmp%2Fsample%20repo")
        .expect("forward valid repository link");

    assert_eq!(
        primary.drain(),
        vec![DeepLinkEvent::OpenRepository("/tmp/sample repo".into())]
    );
}

#[test]
fn secondary_instance_without_a_link_requests_window_activation() {
    let data_dir = tempfile::tempdir().expect("temporary app data directory");
    let primary = DeepLinkInbox::open(data_dir.path()).expect("acquire primary instance");
    let secondary = DeepLinkInbox::open(data_dir.path()).expect("open secondary instance");

    secondary.activate().expect("forward activation request");
    assert_eq!(primary.drain(), vec![DeepLinkEvent::Activate]);
}

#[test]
fn inbox_rejects_invalid_or_overlong_links_without_queuing_them() {
    let data_dir = tempfile::tempdir().expect("temporary app data directory");
    let primary = DeepLinkInbox::open(data_dir.path()).expect("acquire primary instance");
    let secondary = DeepLinkInbox::open(data_dir.path()).expect("open secondary instance");

    assert!(secondary.forward("https://example.com/").is_err());
    let overlong = format!(
        "rocinante://repository/open?path={}",
        "a".repeat(MAX_DEEP_LINK_BYTES)
    );
    assert!(secondary.forward(&overlong).is_err());
    assert!(primary.drain().is_empty());
}

#[test]
fn inbox_caps_the_total_number_of_pending_events() {
    let data_dir = tempfile::tempdir().expect("temporary app data directory");
    let primary = DeepLinkInbox::open(data_dir.path()).expect("acquire primary instance");
    let secondary = DeepLinkInbox::open(data_dir.path()).expect("open secondary instance");

    for _ in 0..MAX_PENDING_DEEP_LINKS {
        secondary.activate().expect("queue within the inbox limit");
    }
    assert_eq!(
        secondary
            .activate()
            .expect_err("reject event beyond the inbox limit")
            .kind(),
        std::io::ErrorKind::WouldBlock
    );
    assert_eq!(primary.drain().len(), MAX_PENDING_DEEP_LINKS);
}

#[test]
fn inbox_drains_sequence_before_lexical_process_id() {
    let data_dir = tempfile::tempdir().expect("temporary app data directory");
    let primary = DeepLinkInbox::open(data_dir.path()).expect("acquire primary instance");
    let queue = data_dir.path().join("deep-link-inbox");
    std::fs::write(
        queue.join("request-10000-00000000000000000002-0.pending"),
        "rocinante://repository/open?path=%2Ftmp%2Fsecond",
    )
    .expect("write second request");
    std::fs::write(
        queue.join("request-9999-00000000000000000001-0.pending"),
        "rocinante://repository/open?path=%2Ftmp%2Ffirst",
    )
    .expect("write first request");

    assert_eq!(
        primary.drain(),
        vec![
            DeepLinkEvent::OpenRepository("/tmp/first".into()),
            DeepLinkEvent::OpenRepository("/tmp/second".into()),
        ]
    );
}

#[test]
fn next_enqueue_cleans_a_temporary_file_left_by_a_crashed_sender() {
    let data_dir = tempfile::tempdir().expect("temporary app data directory");
    let primary = DeepLinkInbox::open(data_dir.path()).expect("acquire primary instance");
    let secondary = DeepLinkInbox::open(data_dir.path()).expect("open secondary instance");
    let orphaned = data_dir.path().join("deep-link-inbox/.request-crashed.tmp");
    std::fs::write(&orphaned, "partial").expect("write stale temporary file");

    secondary.activate().expect("queue an activation event");

    assert!(!orphaned.exists());
    assert_eq!(primary.drain(), vec![DeepLinkEvent::Activate]);
}

#[test]
fn inbox_recovers_sequence_order_from_pending_requests_after_counter_corruption() {
    let data_dir = tempfile::tempdir().expect("temporary app data directory");
    let primary = DeepLinkInbox::open(data_dir.path()).expect("acquire primary instance");
    let queue = data_dir.path().join("deep-link-inbox");
    std::fs::write(queue.join("sequence"), "partial write").expect("corrupt old counter");
    std::fs::write(
        queue.join("request-1234-00000000000000000009-0.pending"),
        "rocinante://repository/open?path=%2Ftmp%2Fold",
    )
    .expect("write older pending link");
    let secondary = DeepLinkInbox::open(data_dir.path()).expect("open secondary instance");

    secondary
        .activate()
        .expect("recover ordering and forward activation");

    assert_eq!(
        primary.drain(),
        vec![
            DeepLinkEvent::OpenRepository("/tmp/old".into()),
            DeepLinkEvent::Activate,
        ]
    );
}

#[test]
fn secondary_process_forwards_deep_link_to_primary() {
    const CHILD_DIRECTORY: &str = "ROCINANTE_DEEP_LINK_CHILD_DIRECTORY";
    if let Some(directory) = std::env::var_os(CHILD_DIRECTORY) {
        let secondary = DeepLinkInbox::open(directory.as_ref()).expect("open secondary inbox");
        assert!(!secondary.is_primary());
        secondary
            .forward("rocinante://repository/open?path=%2Ftmp%2Fchild-process")
            .expect("forward from child process");
        return;
    }

    let data_dir = tempfile::tempdir().expect("temporary app data directory");
    let primary = DeepLinkInbox::open(data_dir.path()).expect("acquire primary instance");
    let output = Command::new(std::env::current_exe().expect("current test executable"))
        .args([
            "--exact",
            "secondary_process_forwards_deep_link_to_primary",
            "--nocapture",
        ])
        .env(CHILD_DIRECTORY, data_dir.path())
        .output()
        .expect("launch secondary test process");
    assert!(
        output.status.success(),
        "secondary process failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        primary.drain(),
        vec![DeepLinkEvent::OpenRepository("/tmp/child-process".into())]
    );
}
