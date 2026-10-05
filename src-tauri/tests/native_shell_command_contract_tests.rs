use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

fn read_repo_file(relative_path: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    fs::read_to_string(path).expect("read repo file")
}

#[test]
fn inventory_matches_every_migrated_command_and_wire_shape_once() {
    let source = read_repo_file("src/command_compat.rs");
    let contract_marker = "pub const MIGRATED_COMMAND_CONTRACTS: [(&str, &str, &str); 11] = [";
    let contract_body = source
        .split_once(contract_marker)
        .expect("migrated command contract")
        .1
        .split_once("];")
        .expect("end of migrated command contract")
        .0;
    let contracts = contract_body
        .split("),")
        .filter_map(|line| {
            let fields = line
                .split('"')
                .enumerate()
                .filter_map(|(index, field)| (index % 2 == 1).then_some(field))
                .collect::<Vec<_>>();
            (fields.len() == 3).then(|| fields.into_iter().map(str::to_string).collect::<Vec<_>>())
        })
        .collect::<Vec<_>>();

    let inventory = read_repo_file("../docs/roadmap/native-shell-command-contract-inventory.md");
    let rows = inventory
        .lines()
        .skip_while(|line| !line.starts_with("| Migrated command |"))
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
            vec![
                command,
                columns[2].replace('`', "").trim().to_string(),
                columns[3].replace('`', "").trim().to_string(),
            ]
        })
        .collect::<Vec<_>>();

    assert_eq!(
        contracts.len(),
        11,
        "all migrated command contracts are recorded"
    );
    assert_eq!(
        rows.len(),
        contracts.len(),
        "inventory has every contract once"
    );
    for (inventory_row, contract) in rows.iter().zip(&contracts) {
        assert_eq!(
            inventory_row, contract,
            "inventory preserves command wire shape"
        );
    }
    let unique = contracts.iter().map(|row| &row[0]).collect::<HashSet<_>>();
    assert_eq!(unique.len(), contracts.len(), "no duplicate command names");
}
