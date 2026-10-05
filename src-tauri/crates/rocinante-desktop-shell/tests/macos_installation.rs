#![cfg(target_os = "macos")]

use std::{os::unix::fs::PermissionsExt, path::PathBuf, process::Command};

#[test]
fn macos_installer_builds_url_handler_bundle_and_registers_it() {
    let temp = tempfile::tempdir().expect("temporary macOS install root");
    let home = temp.path().join("home with spaces");
    let tools = temp.path().join("tools");
    let registration_log = temp.path().join("registration.log");
    let install_name_log = temp.path().join("install-name-tool.log");
    let codesign_log = temp.path().join("codesign.log");
    std::fs::create_dir_all(&home).expect("create temporary home");
    std::fs::create_dir_all(&tools).expect("create command shims");
    let duckdb_library = temp.path().join("deps/libduckdb.dylib");
    std::fs::create_dir_all(duckdb_library.parent().unwrap()).expect("create library fixture");
    std::fs::write(&duckdb_library, b"verified prebuilt DuckDB fixture")
        .expect("write DuckDB library fixture");

    let install_name_tool = tools.join("install_name_tool");
    std::fs::write(
        &install_name_tool,
        "#!/bin/sh\nprintf '%s\\n' \"$*\" >> \"$ROCINANTE_INSTALL_NAME_LOG\"\n",
    )
    .expect("write install_name_tool shim");
    std::fs::set_permissions(&install_name_tool, std::fs::Permissions::from_mode(0o755))
        .expect("make install_name_tool shim executable");

    let otool = tools.join("otool");
    std::fs::write(
        &otool,
        "#!/bin/sh\ncase \"$1\" in\n  -L) printf '%s\\n' \"$ROCINANTE_FAKE_DUCKDB_LOAD_NAME (compatibility version 1.0.0, current version 1.0.0)\" ;;\n  -D) printf '%s\\n' \"$2:\" '/tmp/cache/duckdb-download/x86_64-apple-darwin/1.5.6/libduckdb.dylib' ;;\n  -l) printf '%s\\n' 'Load command 0' ' cmd LC_RPATH' ' cmdsize 40' ' path /tmp/cache/duckdb-download/x86_64-apple-darwin/1.5.6 (offset 12)' ;;\n  *) exit 2 ;;\nesac\n",
    )
    .expect("write otool command shim");
    std::fs::set_permissions(&otool, std::fs::Permissions::from_mode(0o755))
        .expect("make otool shim executable");

    let lsregister = tools.join("lsregister");
    std::fs::write(
        &lsregister,
        "#!/bin/sh\nprintf '%s\\n' \"$*\" > \"$ROCINANTE_LSREGISTER_LOG\"\n",
    )
    .expect("write Launch Services shim");
    std::fs::set_permissions(&lsregister, std::fs::Permissions::from_mode(0o755))
        .expect("make Launch Services shim executable");

    let codesign = tools.join("codesign");
    std::fs::write(
        &codesign,
        "#!/bin/sh\nprintf '%s\\n' \"$*\" >> \"$ROCINANTE_CODESIGN_LOG\"\n",
    )
    .expect("write codesign shim");
    std::fs::set_permissions(&codesign, std::fs::Permissions::from_mode(0o755))
        .expect("make codesign shim executable");

    let binary = temp.path().join("rocinante-desktop-shell");
    std::fs::write(&binary, "#!/bin/sh\nexit 0\n").expect("write fake desktop binary");
    std::fs::set_permissions(&binary, std::fs::Permissions::from_mode(0o755))
        .expect("make fake desktop binary executable");

    let installer =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("packaging/macos/install-user.sh");
    let path = std::env::join_paths(std::iter::once(tools.clone()).chain(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    )))
    .expect("construct command path");
    let output = Command::new("sh")
        .arg(installer)
        .arg(&binary)
        .env("HOME", &home)
        .env("ROCINANTE_LSREGISTER", &lsregister)
        .env("ROCINANTE_LSREGISTER_LOG", &registration_log)
        .env("ROCINANTE_INSTALL_NAME_LOG", &install_name_log)
        .env("ROCINANTE_CODESIGN_LOG", &codesign_log)
        .env("ROCINANTE_CODESIGN_IDENTITY", "contract-identity")
        .env(
            "ROCINANTE_FAKE_DUCKDB_LOAD_NAME",
            "/tmp/cache/duckdb-download/x86_64-apple-darwin/1.5.6/libduckdb.dylib",
        )
        .env("PATH", path)
        .env(
            "ROCINANTE_BUNDLE_IDENTIFIER",
            "dev.rocinante.desktop-shell.contract",
        )
        .output()
        .expect("run macOS user installer");
    assert!(
        output.status.success(),
        "installer failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let bundle = home.join("Applications/Rocinante.app");
    let app_binary = bundle.join("Contents/MacOS/rocinante-desktop-shell");
    let app_icon = bundle.join("Contents/Resources/Rocinante.icns");
    let app_duckdb = bundle.join("Contents/Frameworks/libduckdb.dylib");
    let info_plist = bundle.join("Contents/Info.plist");
    let package_info = bundle.join("Contents/PkgInfo");
    assert!(app_binary.is_file());
    assert!(app_icon.is_file());
    assert_eq!(
        std::fs::read(&app_duckdb).expect("read bundled DuckDB library"),
        b"verified prebuilt DuckDB fixture"
    );
    assert_eq!(
        std::fs::read(&app_binary).expect("read installed executable"),
        std::fs::read(&binary).expect("read source executable")
    );
    let plist = std::fs::read_to_string(&info_plist).expect("read app bundle metadata");
    assert!(plist.contains("<string>rocinante</string>"));
    assert!(plist.contains("<string>dev.rocinante.desktop-shell.contract</string>"));
    assert!(plist.contains("<string>rocinante-desktop-shell</string>"));
    assert!(plist.contains("<key>NSPrincipalClass</key>\n\t<string>NSApplication</string>"));
    assert!(plist.contains("<key>CFBundleIconFile</key>\n\t<string>Rocinante.icns</string>"));
    assert!(plist.contains("<key>LSHandlerRank</key>"));
    assert!(plist.contains("<string>Owner</string>"));
    assert_eq!(
        std::fs::read(package_info).expect("read standard app package metadata"),
        b"APPL????"
    );
    let acceptance_script = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../../scripts/test-macos-url-dispatch.sh"),
    )
    .expect("read bundled macOS URL acceptance script");
    let window_state = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../scripts/macos-window-state.m"),
    )
    .expect("read macOS window-state acceptance helper");
    assert!(window_state.contains("activation_policy="));
    assert!(window_state.contains("active="));
    assert!(acceptance_script.contains("--features acceptance-witness"));
    assert!(acceptance_script.contains("tray_available=true"));
    assert!(acceptance_script.contains("ROCINANTE_ACCEPTANCE_QUIT_FILE"));
    assert!(acceptance_script.contains("ROCINANTE_ACCEPTANCE_CLOSE_FILE"));
    assert!(acceptance_script.contains("ROCINANTE_ACCEPTANCE_SHOW_FILE"));
    assert!(acceptance_script.contains("ROCINANTE_ACCEPTANCE_MINIMIZE_FILE"));
    assert!(acceptance_script.contains("ROCINANTE_ACCEPTANCE_NOTIFICATION=1"));
    assert!(acceptance_script.contains("request_succeeded=true"));
    assert!(acceptance_script.contains("ROCINANTE_ACCEPTANCE_ACTIVATION_RESULT"));
    assert!(acceptance_script.contains("request_accepted="));
    assert!(acceptance_script.contains("ROCINANTE_ACCEPTANCE_MANUAL_TRAY"));
    assert!(acceptance_script.contains("ROCINANTE_ACCEPTANCE_MANUAL_TRAY:-0"));
    assert!(acceptance_script.contains("show_action_started=true"));
    assert!(acceptance_script.contains("wait_for_native_window_state true true"));
    assert!(acceptance_script.contains("choose Show Rocinante"));
    assert!(acceptance_script.contains("choose Quit"));
    assert!(acceptance_script.contains("wait_for_native_window_state false \"*\""));
    assert!(acceptance_script.contains("ROCINANTE_ACCEPTANCE_REQUIRE_FRONTMOST"));
    assert!(acceptance_script.contains("wait_for_native_window_state true \"$expected_frontmost\""));
    assert!(acceptance_script.contains("bundle_process_ids()"));
    assert!(acceptance_script.contains("ROCINANTE_INSTALL_BUNDLE=\"$bundle\""));
    assert!(acceptance_script.contains("ROCINANTE_BUNDLE_IDENTIFIER=\"$bundle_identifier\""));
    assert!(acceptance_script.contains("open \"$cold_uri\""));
    assert!(acceptance_script.contains("open \"$warm_uri\""));
    assert!(acceptance_script.contains("\"$restarted_instance\" == \"$test_instance\""));
    let shell_source =
        std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs"))
            .expect("read native shell startup source");
    assert!(shell_source.contains("record_acceptance_state(&state, tray_icon.is_some())"));
    assert!(shell_source.contains("initial_visibility_requested: false"));
    assert!(shell_source.contains("if !self.initial_visibility_requested"));
    assert!(shell_source.contains("let _ = super::macos_url::activate_application();"));
    assert!(
        shell_source.contains("repaint.send_viewport_cmd(egui::ViewportCommand::Visible(true));")
    );
    assert!(shell_source.contains("repaint.send_viewport_cmd(egui::ViewportCommand::Focus);"));
    let early_registration = shell_source
        .find("super::macos_url::register(url_event_sender, repaint_context.clone())")
        .expect("register URL handler before native event loop startup");
    let app_creation = shell_source
        .find("eframe::run_native(")
        .expect("start native event loop");
    assert!(early_registration < app_creation);
    let macos_url_source =
        std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/macos_url.rs"))
            .expect("read macOS URL handler source");
    assert!(macos_url_source.contains("ViewportCommand::Visible(true)"));
    assert!(macos_url_source.contains("ViewportCommand::Minimized(false)"));
    assert!(macos_url_source.contains("ViewportCommand::Focus"));
    assert!(macos_url_source.contains("ActivateAllWindows"));
    assert!(macos_url_source.contains("activateWithOptions"));
    assert!(macos_url_source.contains("NSApplication::sharedApplication(main_thread).activate()"));
    assert!(shell_source.contains("MenuEvent::set_event_handler(Some(move |event: MenuEvent|"));
    assert!(shell_source.contains("tray_action_sender.send(action)"));
    assert!(shell_source.contains("apply_tray_menu_action(action, &ctx)"));
    assert!(shell_source.contains("start_acceptance_control_watcher"));
    assert!(shell_source.contains("ROCINANTE_ACCEPTANCE_NOTIFICATION_RESULT"));
    assert!(shell_source.contains("super::macos_url::activate_application()"));
    assert!(shell_source.contains("ROCINANTE_ACCEPTANCE_ACTIVATION_RESULT"));
    assert!(acceptance_script.contains("kill -KILL \"$process_id\""));
    assert!(acceptance_script.contains("packaging/macos/install-user.sh"));
    assert!(acceptance_script.contains(
        r#"if [[ "$manual_tray_acceptance" != "1" ]]; then
  rm -f "$quit_file"
fi

: > "$witness""#
    ));
    assert!(acceptance_script.contains("open -a \"$bundle\"\nwait_for_applied_path \"$warm_path\""));
    assert!(acceptance_script.contains("restarted_pid=\"$(sed -n 's/^pid=//p' \"$witness\")\""));
    let validation = Command::new("plutil")
        .args(["-lint"])
        .arg(&info_plist)
        .output()
        .expect("validate app Info.plist");
    assert!(
        validation.status.success(),
        "invalid Info.plist: {}",
        String::from_utf8_lossy(&validation.stderr)
    );
    assert_eq!(
        std::fs::read_to_string(registration_log).expect("read Launch Services registration"),
        format!("-f {}\n", bundle.display())
    );
    let install_name_calls =
        std::fs::read_to_string(install_name_log).expect("read install_name_tool calls");
    assert!(install_name_calls.contains("-id @rpath/libduckdb.dylib"));
    assert!(install_name_calls.contains(
        "-change /tmp/cache/duckdb-download/x86_64-apple-darwin/1.5.6/libduckdb.dylib @rpath/libduckdb.dylib"
    ));
    assert!(install_name_calls.contains(
        "-rpath /tmp/cache/duckdb-download/x86_64-apple-darwin/1.5.6 @executable_path/../Frameworks"
    ));

    let codesign_calls = std::fs::read_to_string(codesign_log)
        .expect("read codesign calls")
        .lines()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    assert_eq!(codesign_calls.len(), 4, "{codesign_calls:?}");
    assert!(codesign_calls[0]
        .contains("--force --options runtime --sign contract-identity --timestamp "));
    assert!(codesign_calls[0].ends_with(app_duckdb.to_str().unwrap()));
    assert!(codesign_calls[1]
        .contains("--force --options runtime --sign contract-identity --timestamp "));
    assert!(codesign_calls[1].ends_with(app_binary.to_str().unwrap()));
    assert!(codesign_calls[2]
        .contains("--force --options runtime --sign contract-identity --timestamp "));
    assert!(codesign_calls[2].ends_with(bundle.to_str().unwrap()));
    assert!(codesign_calls[3].starts_with("--verify --deep --strict "));
    assert!(codesign_calls[3].ends_with(bundle.to_str().unwrap()));
}
