use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

fn read_repo_file(relative_path: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    fs::read_to_string(path).expect("read repo file")
}

#[test]
fn inventory_matches_every_registered_tauri_command_once() {
    let source = read_repo_file("src/app_support.rs");
    let handler_marker = ".invoke_handler(tauri::generate_handler![";
    let handler_body = source
        .split_once(handler_marker)
        .expect("registered handler table")
        .1
        .split_once("])")
        .expect("end of registered handler table")
        .0;
    let registered = handler_body
        .split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
        .filter(|identifier| !identifier.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();

    let inventory = read_repo_file("../docs/roadmap/native-shell-command-contract-inventory.md");
    let rows = inventory
        .lines()
        .skip_while(|line| !line.starts_with("| Registered command |"))
        .skip(2)
        .take_while(|line| line.starts_with("| `"))
        .map(|line| {
            let columns = line.split('|').collect::<Vec<_>>();
            assert_eq!(columns.len(), 6, "inventory row has four columns: {line}");
            let command = columns[1].trim().trim_matches('`').to_string();
            assert!(
                !columns[2].trim().is_empty(),
                "request is documented: {line}"
            );
            assert!(
                !columns[3].trim().is_empty(),
                "response is documented: {line}"
            );
            assert!(
                !columns[4].trim().is_empty(),
                "service owner is documented: {line}"
            );
            command
        })
        .collect::<Vec<_>>();

    assert_eq!(registered.len(), 11, "handler registration count changed");
    assert_eq!(
        rows.len(),
        registered.len(),
        "inventory has every handler once"
    );
    assert_eq!(
        rows, registered,
        "inventory follows the registered command list"
    );
    let unique = rows.iter().collect::<HashSet<_>>();
    assert_eq!(
        unique.len(),
        rows.len(),
        "inventory has no duplicate command"
    );
}
