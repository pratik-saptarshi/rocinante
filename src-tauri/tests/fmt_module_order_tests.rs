#[test]
fn src_tauri_lib_mods_are_sorted_alphabetically() {
    let lib_contents = include_str!("../src/lib.rs");

    let modules: Vec<&str> = lib_contents
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if !trimmed.starts_with("pub mod ") || !trimmed.ends_with(';') {
                return None;
            }
            trimmed
                .strip_prefix("pub mod ")?
                .strip_suffix(';')
                .map(str::trim)
        })
        .collect();

    let mut sorted = modules.clone();
    sorted.sort_unstable();
    assert_eq!(
        modules, sorted,
        "src-tauri/src/lib.rs module declarations are not alphabetical"
    );
}
