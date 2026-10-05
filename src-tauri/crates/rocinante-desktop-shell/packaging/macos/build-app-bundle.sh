#!/bin/sh
set -eu

if [ "$#" -ne 2 ]; then
    echo "usage: build-app-bundle.sh PATH_TO_EXECUTABLE PATH_TO_OUTPUT_APP" >&2
    exit 2
fi

source_binary=$1
output_app=$2
bundle_identifier=${ROCINANTE_BUNDLE_IDENTIFIER:-dev.rocinante.desktop-shell}
case "$bundle_identifier" in
    ''|*[!A-Za-z0-9.-]*)
        echo "invalid macOS bundle identifier: $bundle_identifier" >&2
        exit 2
        ;;
esac
if [ ! -f "$source_binary" ] || [ ! -x "$source_binary" ]; then
    echo "desktop shell binary must be an executable file: $source_binary" >&2
    exit 2
fi

contents=$output_app/Contents
source_directory=$(CDPATH='' cd -- "$(dirname -- "$source_binary")" && pwd -P)
duckdb_library=$source_directory/deps/libduckdb.dylib
if [ ! -f "$duckdb_library" ]; then
    echo "verified prebuilt DuckDB shared library is missing: $duckdb_library" >&2
    exit 1
fi
for tool in install_name_tool otool codesign; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        echo "$tool is required to build and sign the app bundle" >&2
        exit 1
    fi
done
code_sign_identity=${ROCINANTE_CODESIGN_IDENTITY:--}
script_directory=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
mkdir -p "$contents/MacOS" "$contents/Resources" "$contents/Frameworks"
installed_binary=$contents/MacOS/rocinante-desktop-shell
installed_duckdb=$contents/Frameworks/libduckdb.dylib
install -m 0755 "$source_binary" "$installed_binary"
install -m 0755 "$duckdb_library" "$installed_duckdb"
duckdb_id=$(otool -D "$installed_duckdb" | sed -n '2p')
if [ "$duckdb_id" != @rpath/libduckdb.dylib ]; then
    install_name_tool -id @rpath/libduckdb.dylib "$installed_duckdb"
fi

duckdb_load_name=$(otool -L "$installed_binary" | sed -n \
    '/libduckdb\.dylib (compatibility version/{s/ (compatibility version.*//;s/^[[:space:]]*//;p;}')
if [ -z "$duckdb_load_name" ]; then
    echo "the desktop shell does not link the expected prebuilt libduckdb.dylib" >&2
    exit 1
fi
if [ "$duckdb_load_name" != @rpath/libduckdb.dylib ]; then
    install_name_tool -change "$duckdb_load_name" @rpath/libduckdb.dylib "$installed_binary"
fi

duckdb_rpaths=$(otool -l "$installed_binary" | awk '
    /cmd LC_RPATH/ { getline; getline; sub(/^[[:space:]]*path /, ""); sub(/ [(]offset.*/, ""); print }
')
has_framework_rpath=false
while IFS= read -r rpath; do
    [ -n "$rpath" ] || continue
    case "$rpath" in
        *duckdb-download*)
            install_name_tool -rpath "$rpath" @executable_path/../Frameworks "$installed_binary"
            has_framework_rpath=true
            ;;
        @executable_path/../Frameworks)
            has_framework_rpath=true
            ;;
    esac
done <<EOF
$duckdb_rpaths
EOF
if [ "$has_framework_rpath" != true ]; then
    install_name_tool -add_rpath @executable_path/../Frameworks "$installed_binary"
fi
sed "s|<string>dev.rocinante.desktop-shell</string>|<string>$bundle_identifier</string>|" \
    "$script_directory/Info.plist" > "$contents/Info.plist"
install -m 0644 "$script_directory/../icons/Rocinante.icns" "$contents/Resources/Rocinante.icns"
printf 'APPL????' > "$contents/PkgInfo"

sign_code() {
    if [ "$code_sign_identity" = "-" ]; then
        codesign --force --sign "$code_sign_identity" --timestamp=none "$1"
    else
        codesign --force --options runtime --sign "$code_sign_identity" --timestamp "$1"
    fi
}

# install_name_tool changes Mach-O signatures, so sign every nested code object
# and the completed bundle after all loader paths and bundle metadata are final.
sign_code "$installed_duckdb"
sign_code "$installed_binary"
sign_code "$output_app"
codesign --verify --deep --strict "$output_app"
