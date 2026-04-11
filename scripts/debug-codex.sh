#!/bin/bash

# Set "chatgpt.cliExecutable": "/Users/<USERNAME>/code/codex/scripts/debug-codex.sh" in VSCode settings to always get the 
# latest codex-rs binary when debugging Codex Extension.


set -euo pipefail

CODEX_RS_DIR=$(realpath "$(dirname "$0")/../codex-rs")
CODEX_BIN="$CODEX_RS_DIR/target/debug/codex"

if [[ ! -x "$CODEX_BIN" ]]; then
  echo "missing local codex binary: $CODEX_BIN" >&2
  echo "build it manually before using this debug wrapper" >&2
  exit 1
fi

exec "$CODEX_BIN" "$@"
