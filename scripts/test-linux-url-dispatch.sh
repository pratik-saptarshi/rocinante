#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
shell_manifest="$repo_root/src-tauri/crates/rocinante-desktop-shell/Cargo.toml"
shell_binary="$repo_root/src-tauri/target/debug/rocinante-desktop-shell"
installer="$repo_root/src-tauri/crates/rocinante-desktop-shell/packaging/linux/install-user.sh"
test_root="$(mktemp -d "${TMPDIR:-/tmp}/rocinante-linux-url-acceptance.XXXXXX")"
app_data="$test_root/app-data"
state_file="$app_data/shell-state.ron"
state_json_file="$app_data/shell-state.json"
witness="$test_root/applied-state.txt"
quit_file="$test_root/request-clean-quit"
notification_log="$test_root/notification-log.txt"
notification_result="$test_root/notification-request.txt"
dunst_config="$test_root/dunst.conf"
test_pid=""
dunst_pid=""

installed_binary=""
proc_matches_binary() {
  local process_id=$1 current_binary
  [[ -n "$installed_binary" && -e "/proc/$process_id/exe" ]] || return 1
  current_binary="$(readlink -f "/proc/$process_id/exe" 2>/dev/null || true)"
  [[ "$current_binary" == "$installed_binary" ]]
}

binary_process_ids() {
  local proc_exe process_id
  for proc_exe in /proc/[0-9]*/exe; do
    [[ -e "$proc_exe" ]] || continue
    process_id="${proc_exe#/proc/}"
    process_id="${process_id%/exe}"
    proc_matches_binary "$process_id" && printf '%s\n' "$process_id"
  done
}

cleanup() {
  local process_id process_ids
  process_ids="$test_pid $(binary_process_ids)"
  if [[ -n "$installed_binary" ]]; then
    touch "$quit_file"
    for process_id in $process_ids; do
      [[ -n "$process_id" ]] || continue
      for _ in {1..20}; do
        proc_matches_binary "$process_id" || break
        sleep 1
      done
      if proc_matches_binary "$process_id"; then
        kill -TERM "$process_id" 2>/dev/null || true
        for _ in {1..10}; do
          proc_matches_binary "$process_id" || break
          sleep 1
        done
        proc_matches_binary "$process_id" && kill -KILL "$process_id" 2>/dev/null || true
      fi
    done
  fi
  if [[ -n "$dunst_pid" ]] && kill -0 "$dunst_pid" 2>/dev/null; then
    kill "$dunst_pid" 2>/dev/null || true
    wait "$dunst_pid" 2>/dev/null || true
  fi
  rm -rf "$test_root"
}
trap cleanup EXIT

if [[ "$(uname -s)" != "Linux" ]]; then
  echo "Linux desktop URI acceptance requires Linux" >&2
  exit 2
fi
if [[ -z "${DISPLAY:-}" || -z "${DBUS_SESSION_BUS_ADDRESS:-}" ]]; then
  echo "Run this acceptance under Xvfb and a private D-Bus session" >&2
  exit 2
fi
for command_name in cargo dunst dunstctl gio xdg-mime; do
  if ! command -v "$command_name" >/dev/null 2>&1; then
    echo "Required command is unavailable: $command_name" >&2
    exit 2
  fi
done

mkdir -p "$app_data"
ROCINANTE_ACCEPTANCE_WITNESS="$witness" \
ROCINANTE_ACCEPTANCE_DATA_DIR="$app_data" \
ROCINANTE_ACCEPTANCE_QUIT_FILE="$quit_file" \
ROCINANTE_ACCEPTANCE_NOTIFICATION=1 \
ROCINANTE_ACCEPTANCE_NOTIFICATION_RESULT="$notification_result" \
  cargo build --manifest-path "$shell_manifest" --bin rocinante-desktop-shell \
    --features acceptance-witness --locked
if [[ ! -x "$shell_binary" ]]; then
  echo "Build did not produce executable $shell_binary" >&2
  exit 1
fi

export HOME="$test_root/home\$with-dollar"
export XDG_DATA_HOME="$test_root/xdg-data"
export XDG_CONFIG_HOME="$test_root/xdg-config"
export XDG_CACHE_HOME="$test_root/xdg-cache"
export WINIT_UNIX_BACKEND=x11
export WGPU_BACKEND=gl
export LIBGL_ALWAYS_SOFTWARE=1
mkdir -p "$HOME" "$XDG_DATA_HOME" "$XDG_CONFIG_HOME" "$XDG_CACHE_HOME"
"$installer" "$shell_binary"
desktop-file-validate "$XDG_DATA_HOME/applications/rocinante.desktop"
installed_binary="$HOME/.local/bin/rocinante-desktop-shell"
if [[ "$(xdg-mime query default x-scheme-handler/rocinante)" != "rocinante.desktop" ]]; then
  echo "The per-user desktop database did not select rocinante.desktop for the URI scheme" >&2
  exit 1
fi
cat > "$dunst_config" <<'EOF'
[global]
format = "%s"
timeout = 120
EOF
dunst --config "$dunst_config" --print > "$notification_log" 2>&1 &
dunst_pid=$!
for _ in {1..30}; do
  if ! kill -0 "$dunst_pid" 2>/dev/null; then
    cat "$notification_log" >&2
    echo "The test notification daemon exited during startup" >&2
    exit 1
  fi
  notification_name_owned="$(dbus-send --session --type=method_call --print-reply \
    --dest=org.freedesktop.DBus /org/freedesktop/DBus \
    org.freedesktop.DBus.NameHasOwner string:org.freedesktop.Notifications \
    2>/dev/null | sed -n 's/.*boolean //p')"
  if [[ "$notification_name_owned" == true ]]; then break; fi
  sleep 1
done
if [[ "$notification_name_owned" != true ]]; then
  echo "The test notification daemon did not register on the private D-Bus session" >&2
  exit 1
fi

wait_for_applied_path() {
  local expected_path=$1 witness_pid
  for _ in {1..90}; do
    if [[ -f "$witness" ]]; then
      witness_pid="$(sed -n 's/^pid=//p' "$witness")"
      [[ -z "$witness_pid" ]] || test_pid="$witness_pid"
      if grep -F -x "page=Repositories" "$witness" >/dev/null \
        && grep -F -x "path=$expected_path" "$witness" >/dev/null; then
        return 0
      fi
    fi
    sleep 1
  done
  echo "The installed desktop handler did not apply URI target: $expected_path" >&2
  if [[ -f "$witness" ]]; then cat "$witness" >&2; else echo '<no witness>' >&2; fi
  return 1
}

wait_for_notification() {
  for _ in {1..30}; do
    if grep -F "Rocinante notification acceptance" "$notification_log" >/dev/null; then
      return 0
    fi
    sleep 1
  done
  echo "The desktop notification daemon did not receive the shell's notification" >&2
  cat "$notification_log" >&2
  return 1
}

assert_notification_is_displayed() {
  local displayed_count
  displayed_count="$(dunstctl count displayed)"
  if [[ ! "$displayed_count" =~ ^[0-9]+$ ]] || (( displayed_count < 1 )); then
    echo "Dunst received the notification but has no displayed notification" >&2
    dunstctl history >&2 || true
    return 1
  fi
}

make_uri() {
  python3 -c 'from urllib.parse import quote; import sys; print("rocinante://repository/open?path=" + quote(sys.argv[1], safe=""))' "$1"
}

cold_path="$test_root/cold repository"
warm_path="$test_root/warm repository"
mkdir -p "$cold_path" "$warm_path"
gio open "$(make_uri "$cold_path")"
wait_for_applied_path "$cold_path"
wait_for_notification
if ! grep -F -x "request_succeeded=true" "$notification_result" >/dev/null 2>&1; then
  echo "The shell did not successfully submit its acceptance notification" >&2
  [[ ! -f "$notification_result" ]] || cat "$notification_result" >&2
  exit 1
fi
assert_notification_is_displayed
test_pid="$(sed -n 's/^pid=//p' "$witness")"
test_instance="$(sed -n 's/^instance=//p' "$witness")"
if [[ -z "$test_pid" ]]; then
  echo "The cold-launched shell did not report its process id" >&2
  exit 1
fi
if [[ -z "$test_instance" ]]; then
  echo "The cold-launched shell did not report its instance id" >&2
  exit 1
fi

gio open "$(make_uri "$warm_path")"
wait_for_applied_path "$warm_path"
if [[ "$(sed -n 's/^pid=//p' "$witness")" != "$test_pid" ]]; then
  echo "Warm URI delivery did not reach the cold-launched process" >&2
  exit 1
fi

touch "$quit_file"
for _ in {1..60}; do
  proc_matches_binary "$test_pid" || break
  sleep 1
done
if proc_matches_binary "$test_pid"; then
  echo "The installed shell did not exit after the clean-quit request" >&2
  exit 1
fi
rm -f "$quit_file"
if [[ ! -s "$state_file" ]]; then
  echo "The app did not persist eframe state before exit" >&2
  exit 1
fi
if [[ ! -s "$state_json_file" ]]; then
  echo "The app did not persist its restart state before exit" >&2
  exit 1
fi
echo "Persisted shell state: $(cat "$state_file")"

: > "$witness"
"$installed_binary" >/dev/null 2>&1 &
wait_for_applied_path "$warm_path"
restarted_pid="$(sed -n 's/^pid=//p' "$witness")"
restarted_instance="$(sed -n 's/^instance=//p' "$witness")"
if [[ -z "$restarted_pid" || -z "$restarted_instance" || "$restarted_instance" == "$test_instance" ]]; then
  echo "The installed shell did not restore saved state in a new process. Witness: $(cat "$witness" 2>/dev/null || echo '<no witness>')" >&2
  exit 1
fi
test_pid="$restarted_pid"

echo "Linux registered cold/warm URI delivery and saved-state restart passed (pid $test_pid)."
