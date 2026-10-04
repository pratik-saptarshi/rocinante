#![cfg(feature = "native-ui")]

#[test]
fn windows_installer_registers_current_user_url_handler() {
    let installer = include_str!("../packaging/windows/install-user.ps1");
    let acceptance = include_str!("../../../../scripts/test-windows-registration.ps1");

    assert!(installer.contains("HKCU:\\Software\\Classes\\rocinante"));
    assert!(installer.contains("URL Protocol"));
    assert!(installer.contains("Programs\\Rocinante"));
    assert!(installer.contains("rocinante-desktop-shell.exe"));
    assert!(installer.contains("Rocinante.ico"));
    assert!(installer.contains("DefaultIcon"));
    assert!(installer.contains("CreateShortcut($shortcutPath)"));
    assert!(installer.contains("$shortcut.IconLocation = $iconLocation"));
    assert!(installer.contains("$command = '\"' + $installedBinary + '\" \"%1\"'"));
    assert!(installer.contains("$sourceDirectory 'deps\\duckdb.dll'"));
    assert!(installer.contains("duckdb.dll"));
    assert!(acceptance.contains("scripts\\provision_duckdb.py"));
    assert!(acceptance.contains("x86_64-pc-windows-msvc"));
    assert!(acceptance.contains("Get-FileHash -LiteralPath $installedDuckdb -Algorithm SHA256"));
    assert!(!installer.contains("HKLM:"));
}

#[test]
fn windows_url_acceptance_covers_dispatch_restart_and_cleanup() {
    let acceptance = include_str!("../../../../scripts/test-windows-url-dispatch.ps1");

    for required in [
        "cargo build --manifest-path $manifest",
        "Start-Process -FilePath (New-RepositoryUri $coldPath)",
        "Start-Process -FilePath $installedBinary -ArgumentList (New-RepositoryUri $warmPath) -PassThru",
        "The app did not persist eframe state before exit",
        "ROCINANTE_ACCEPTANCE_FORWARD_WITNESS",
        "forward-attempt.txt",
        "if ($warmPid -ne $primaryPid)",
        "$primaryInstance = ((Get-Content -LiteralPath $witness | Where-Object { $_ -like 'instance=*' }) -replace '^instance=', '')",
        "request-clean-quit",
        "Remove-Item -LiteralPath $quitFile -Force",
        "Start-Process -FilePath $installedBinary",
        "if (-not $restartedPid -or -not $restartedInstance -or $restartedInstance -eq $primaryInstance)",
        "HKCU:\\Software\\Classes\\rocinante",
        "Stop-Process -Id $processId -Force",
    ] {
        assert!(
            acceptance.contains(required),
            "missing acceptance step: {required}"
        );
    }
    assert!(
        acceptance
            .find("Remove-Item -LiteralPath $quitFile -Force")
            .unwrap()
            < acceptance
                .rfind("Start-Process -FilePath $installedBinary")
                .unwrap(),
        "the restarted process must not inherit the previous process's clean-quit request"
    );
}
