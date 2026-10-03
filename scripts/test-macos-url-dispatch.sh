#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
shell_manifest="$repo_root/src-tauri/crates/rocinante-desktop-shell/Cargo.toml"
shell_binary="$repo_root/src-tauri/target/debug/rocinante-desktop-shell"
bundle_root="$(mktemp -d "${TMPDIR:-/tmp}/rocinante-url-acceptance.XXXXXX")"
bundle_root="$(cd "$bundle_root" && pwd -P)"
bundle="${HOME:?}/Applications/RocinanteAcceptance-$$.app"
bundle_identifier="dev.rocinante.desktop-shell.acceptance.$$"
bundle_binary="$bundle/Contents/MacOS/rocinante-desktop-shell"
app_data="$bundle_root/app-data"
witness="$bundle_root/applied-state.txt"
notification_result="$bundle_root/notification-request.txt"
activation_result="$bundle_root/activation-request.txt"
quit_file="$bundle_root/request-clean-quit"
close_file="$bundle_root/request-window-close"
show_file="$bundle_root/request-tray-show"
minimize_file="$bundle_root/request-minimize"
window_state_helper="$bundle_root/macos-window-state"
expected_frontmost="*"
if [[ "${ROCINANTE_ACCEPTANCE_REQUIRE_FRONTMOST:-0}" == "1" ]]; then
  expected_frontmost=true
fi
lsregister="/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister"
test_pid=""

bundle_process_ids() {
  ps -A -o pid= -o command= 2>/dev/null | awk -v executable="$bundle_binary" \
    'index($0, executable) > 0 { print $1 }' || true
}

bundle_pid_is_running() {
  bundle_process_ids | grep -F -x "$1" >/dev/null 2>&1
}

cleanup() {
  process_ids="$test_pid $(bundle_process_ids)"
  for process_id in $process_ids; do
    [[ -n "$process_id" ]] || continue
    kill -0 "$process_id" 2>/dev/null || continue
    touch "$quit_file"
    for _ in {1..45}; do
      kill -0 "$process_id" 2>/dev/null || break
      sleep 1
    done
    command_line="$(ps -p "$process_id" -o command= 2>/dev/null || true)"
    if [[ "$command_line" == *"$bundle_binary"* ]]; then
      kill -TERM "$process_id" 2>/dev/null || true
      for _ in {1..30}; do
        kill -0 "$process_id" 2>/dev/null || break
        sleep 1
      done
      command_line="$(ps -p "$process_id" -o command= 2>/dev/null || true)"
      if [[ "$command_line" == *"$bundle_binary"* ]]; then
        kill -KILL "$process_id" 2>/dev/null || true
        for _ in {1..10}; do
          kill -0 "$process_id" 2>/dev/null || break
          sleep 1
        done
      fi
    fi
  done
  "$lsregister" -u "$bundle" >/dev/null 2>&1 || true
  rm -rf "$bundle"
  rm -rf "$bundle_root"
}
trap cleanup EXIT

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "macOS Launch Services acceptance requires Darwin" >&2
  exit 2
fi
if pgrep -x rocinante-desktop-shell >/dev/null 2>&1; then
  echo "Close any running Rocinante desktop shell before this acceptance test" >&2
  exit 2
fi

clang "$repo_root/scripts/macos-window-state.m" \
  -framework AppKit -framework CoreGraphics -o "$window_state_helper"
mkdir -p "$app_data" "$(dirname "$bundle")"
ROCINANTE_ACCEPTANCE_WITNESS="$witness" \
ROCINANTE_ACCEPTANCE_DATA_DIR="$app_data" \
ROCINANTE_ACCEPTANCE_NOTIFICATION=1 \
ROCINANTE_ACCEPTANCE_NOTIFICATION_RESULT="$notification_result" \
ROCINANTE_ACCEPTANCE_ACTIVATION_RESULT="$activation_result" \
ROCINANTE_ACCEPTANCE_QUIT_FILE="$quit_file" \
ROCINANTE_ACCEPTANCE_CLOSE_FILE="$close_file" \
ROCINANTE_ACCEPTANCE_SHOW_FILE="$show_file" \
ROCINANTE_ACCEPTANCE_MINIMIZE_FILE="$minimize_file" \
  cargo build --manifest-path "$shell_manifest" --bin rocinante-desktop-shell \
    --features acceptance-witness --locked
ROCINANTE_INSTALL_BUNDLE="$bundle" \
ROCINANTE_BUNDLE_IDENTIFIER="$bundle_identifier" \
  sh "$repo_root/src-tauri/crates/rocinante-desktop-shell/packaging/macos/install-user.sh" \
    "$shell_binary"
registration_dump="$bundle_root/launch-services.dump"
"$lsregister" -dump > "$registration_dump"
if ! grep -F "$bundle" "$registration_dump" >/dev/null \
  || ! grep -F "$bundle_identifier" "$registration_dump" >/dev/null; then
  echo "Launch Services did not retain the installed acceptance bundle" >&2
  grep -n -i -C 2 -E 'rocinante|acceptance' "$registration_dump" >&2 || true
  exit 1
fi
if [[ ! -f "$bundle/Contents/Resources/Rocinante.icns" ]]; then
  echo "The bundled application icon is missing" >&2
  exit 1
fi
wait_for_applied_path() {
  expected_path=$1
  for _ in {1..45}; do
    if [[ -f "$witness" ]]; then
      witness_pid="$(sed -n 's/^pid=//p' "$witness")"
      if [[ -n "$witness_pid" ]]; then
        test_pid="$witness_pid"
      fi
    fi
    if [[ -f "$witness" ]] \
      && grep -F -x "page=Repositories" "$witness" >/dev/null \
      && grep -F -x "path=$expected_path" "$witness" >/dev/null \
      && grep -F -x "tray_available=true" "$witness" >/dev/null; then
      return 0
    fi
    sleep 1
  done
  echo "The bundled app did not apply the URI target: $expected_path" >&2
  if [[ -f "$witness" ]]; then
    cat "$witness" >&2
  fi
  return 1
}

wait_for_native_window_state() {
  expected_visible=$1
  expected_frontmost=$2
  for _ in {1..45}; do
    window_state="$("$window_state_helper" "$test_pid" 2>/dev/null || true)"
    if grep -F -x "visible=$expected_visible" <<< "$window_state" >/dev/null \
      && { [[ "$expected_frontmost" == "*" ]] \
        || grep -F -x "frontmost=$expected_frontmost" <<< "$window_state" >/dev/null; }; then
      return 0
    fi
    sleep 1
  done
  echo "The bundled app did not reach visible=$expected_visible frontmost=$expected_frontmost" >&2
  "$window_state_helper" "$test_pid" >&2 || true
  return 1
}

make_uri() {
  python3 -c 'from urllib.parse import quote; import sys; print("rocinante://repository/open?path=" + quote(sys.argv[1], safe=""))' "$1"
}

cold_path="$bundle_root/cold-repository"
warm_path="$bundle_root/warm-repository"
mkdir -p "$cold_path" "$warm_path"
cold_uri="$(make_uri "$cold_path")"
open "$cold_uri"
wait_for_applied_path "$cold_path"
test_pid="$(sed -n 's/^pid=//p' "$witness")"
test_instance="$(sed -n 's/^instance=//p' "$witness")"
if [[ -z "$test_pid" ]]; then
  echo "The bundled app did not report its process id" >&2
  exit 1
fi
if [[ -z "$test_instance" ]]; then
  echo "The bundled app did not report its process instance" >&2
  exit 1
fi

wait_for_native_window_state true "$expected_frontmost"
for _ in {1..45}; do
  [[ -f "$notification_result" ]] && break
  sleep 1
done
if ! grep -F -x "request_succeeded=true" "$notification_result" >/dev/null 2>&1; then
  echo "The native notification request did not succeed" >&2
  [[ ! -f "$notification_result" ]] || cat "$notification_result" >&2
  exit 1
fi
touch "$close_file"
for _ in {1..45}; do
  [[ -e "$close_file.received" ]] && break
  sleep 1
done
if [[ ! -e "$close_file.received" ]]; then
  echo "The native UI did not consume the window-close request" >&2
  exit 1
fi
wait_for_native_window_state false "*"
if ! kill -0 "$test_pid" 2>/dev/null; then
  echo "Closing the window exited the process instead of hiding it to the tray" >&2
  exit 1
fi

touch "$show_file"
for _ in {1..45}; do
  [[ -e "$show_file.received" ]] && break
  sleep 1
done
if [[ ! -e "$show_file.received" ]]; then
  echo "The native UI did not consume the tray Show request" >&2
  exit 1
fi
for _ in {1..45}; do
  if grep -Eq '^request_accepted=(true|false)$' "$activation_result" 2>/dev/null; then
    break
  fi
  kill -0 "$test_pid" 2>/dev/null || break
  sleep 1
done
if ! grep -Eq '^request_accepted=(true|false)$' "$activation_result" 2>/dev/null; then
  echo "The native Show handler did not complete its AppKit activation request" >&2
  if [[ -f "$activation_result" ]]; then
    cat "$activation_result" >&2
  fi
  exit 1
fi
activation_request="$(sed -n 's/^request_accepted=//p' "$activation_result")"
if [[ "$activation_request" != true && "$activation_request" != false ]]; then
  echo "The native Show handler reported an invalid AppKit activation result" >&2
  cat "$activation_result" >&2
  exit 1
fi
wait_for_native_window_state true "$expected_frontmost"

touch "$minimize_file"
for _ in {1..45}; do
  [[ -e "$minimize_file.received" ]] && break
  sleep 1
done
if [[ ! -e "$minimize_file.received" ]]; then
  echo "The native UI did not consume the minimize request" >&2
  exit 1
fi
wait_for_native_window_state false "*"

warm_uri="$(make_uri "$warm_path")"
open "$warm_uri"
wait_for_applied_path "$warm_path"
wait_for_native_window_state true "$expected_frontmost"
if [[ "$(sed -n 's/^pid=//p' "$witness")" != "$test_pid" \
  || "$(sed -n 's/^instance=//p' "$witness")" != "$test_instance" ]]; then
  echo "Warm URI delivery started a second process instead of reaching the running app" >&2
  exit 1
fi

touch "$quit_file"
for _ in {1..45}; do
  bundle_pid_is_running "$test_pid" || break
  sleep 1
done
if bundle_pid_is_running "$test_pid"; then
  echo "The bundled app did not exit cleanly after the acceptance quit request" >&2
  exit 1
fi
rm -f "$quit_file"

: > "$witness"
open -a "$bundle"
wait_for_applied_path "$warm_path"
restarted_pid="$(sed -n 's/^pid=//p' "$witness")"
restarted_instance="$(sed -n 's/^instance=//p' "$witness")"
if [[ -z "$restarted_pid" || -z "$restarted_instance" \
  || "$restarted_instance" == "$test_instance" ]]; then
  echo "The bundled app did not restore state in a new process after restart" >&2
  echo "first instance=$test_instance, restarted instance=$restarted_instance" >&2
  cat "$witness" >&2
  exit 1
fi
test_pid="$restarted_pid"

echo "Bundled cold/warm URL delivery, tray lifecycle, notification request, minimized-window visible restore, and saved-state restart passed (pid $test_pid; AppKit activation request accepted=$activation_request)."
