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
script_directory=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
mkdir -p "$contents/MacOS" "$contents/Resources"
install -m 0755 "$source_binary" "$contents/MacOS/rocinante-desktop-shell"
sed "s|<string>dev.rocinante.desktop-shell</string>|<string>$bundle_identifier</string>|" \
    "$script_directory/Info.plist" > "$contents/Info.plist"
install -m 0644 "$script_directory/../icons/Rocinante.icns" "$contents/Resources/Rocinante.icns"
printf 'APPL????' > "$contents/PkgInfo"
