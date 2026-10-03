#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: install-user.sh PATH_TO_ROCINANTE_DESKTOP_SHELL" >&2
    exit 2
fi
if [ -z "${HOME:-}" ]; then
    echo "HOME must be set for a per-user installation" >&2
    exit 2
fi

source_binary=$1
if [ ! -f "$source_binary" ] || [ ! -x "$source_binary" ]; then
    echo "desktop shell binary must be an executable file: $source_binary" >&2
    exit 2
fi

script_directory=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
bundle=${ROCINANTE_INSTALL_BUNDLE:-$HOME/Applications/Rocinante.app}
bundle_identifier=${ROCINANTE_BUNDLE_IDENTIFIER:-dev.rocinante.desktop-shell}
lsregister=${ROCINANTE_LSREGISTER:-/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister}

ROCINANTE_BUNDLE_IDENTIFIER="$bundle_identifier" \
    sh "$script_directory/build-app-bundle.sh" "$source_binary" "$bundle"
if [ ! -x "$lsregister" ]; then
    echo "Launch Services registration tool was not found: $lsregister" >&2
    exit 1
fi
"$lsregister" -f "$bundle"
printf 'Installed Rocinante Repo Analyzer in %s and registered its URL scheme.\n' "$bundle"
