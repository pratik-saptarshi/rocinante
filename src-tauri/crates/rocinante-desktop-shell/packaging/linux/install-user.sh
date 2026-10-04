#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    echo "usage: install-user.sh PATH_TO_ROCINANTE_DESKTOP_SHELL" >&2
    exit 2
fi

source_binary=$1
if [ ! -f "$source_binary" ] || [ ! -x "$source_binary" ]; then
    echo "desktop shell binary must be an executable file: $source_binary" >&2
    exit 2
fi
if [ -z "${HOME:-}" ]; then
    echo "HOME must be set for a per-user installation" >&2
    exit 2
fi

script_directory=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
binary_directory=$HOME/.local/bin
data_directory=${XDG_DATA_HOME:-$HOME/.local/share}
application_directory=$data_directory/applications
icon_directory=$data_directory/icons/hicolor/512x512/apps
mime_directory=$data_directory/mime
installed_binary=$binary_directory/rocinante-desktop-shell
desktop_entry=$application_directory/rocinante.desktop

mkdir -p "$binary_directory" "$application_directory" "$icon_directory"
install -m 0755 "$source_binary" "$installed_binary"
install -m 0644 "$script_directory/../icons/rocinante.png" "$icon_directory/rocinante.png"

# Escape reserved characters for the quoted executable field in a .desktop file.
escaped_binary=$(printf '%s' "$installed_binary" | sed 's/\\/\\\\\\\\/g; s/["`]/\\&/g; s/\$/\\\\$/g; s/%/%%/g')
escaped_try_exec=$(printf '%s' "$installed_binary" | sed 's/\\/\\\\/g; s/ /\\s/g')
temporary_entry=$(mktemp "$application_directory/.rocinante.desktop.XXXXXX")
trap 'rm -f "$temporary_entry"' EXIT HUP INT TERM
{
    sed '/^Exec=/d; /^TryExec=/d' "$script_directory/rocinante.desktop"
    printf 'Exec="%s" %%u\n' "$escaped_binary"
    printf 'TryExec=%s\n' "$escaped_try_exec"
} > "$temporary_entry"
install -m 0644 "$temporary_entry" "$desktop_entry"

if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$application_directory"
fi
if command -v xdg-mime >/dev/null 2>&1; then
    xdg-mime default rocinante.desktop x-scheme-handler/rocinante
fi

printf 'Installed Rocinante desktop shell for this user.\n'
