use rocinante_sled_migration::migrate_legacy_sled_store;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args_os().skip(1);
    let Some(root) = args.next() else {
        print_usage();
        return ExitCode::from(2);
    };
    let help_requested = root == "--help" || root == "-h";
    let has_extra_argument = args.next().is_some();
    if help_requested || has_extra_argument {
        print_usage();
        return if help_requested && !has_extra_argument {
            ExitCode::SUCCESS
        } else {
            ExitCode::from(2)
        };
    }

    match migrate_legacy_sled_store(PathBuf::from(root)) {
        Ok(report) => {
            println!(
                "Sled migration {}: {} trees, {} records, source SHA-256 {}",
                if report.already_migrated {
                    "already complete"
                } else {
                    "complete"
                },
                report.tree_count,
                report.record_count,
                report.source_sha256
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Sled migration failed: {error}");
            ExitCode::FAILURE
        }
    }
}

fn print_usage() {
    eprintln!(
        "usage: rocinante-sled-migration <legacy-sled-directory>\n\
         Stops if Rocinante owns the storage lock. Preserves the original Sled\n\
         files and writes verified records to ingestion.sqlite3 in that directory."
    );
}
