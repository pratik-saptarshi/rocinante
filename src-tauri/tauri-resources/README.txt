This directory is a configured Tauri bundle resource directory.
The build hook stages the checksum-verified DuckDB binary here before Cargo
validates the resource glob. Direct Cargo checks use this marker when no
platform runtime has been staged.
