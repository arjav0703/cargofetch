#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

REPO="arjav0703/cargofetch"
BIN="cargofetch"
INSTALL_DIR="/usr/local/bin"
GITHUB_API="https://api.github.com/repos/${REPO}/releases"

log() { >&2 echo "[$(date +'%H:%M:%S')] $*"; }

# Cleanup temp dir
TMPDIR="$(mktemp -d)"
cleanup() { rm -rf "$TMPDIR"; }
trap cleanup EXIT

ensure_sudo() {
  if (( EUID != 0 )); then
    if command -v sudo &>/dev/null; then
      SUDO="sudo"
    else
      log "error: need root privileges and sudo is not installed."
      exit 1
    fi
  else
    SUDO=""
  fi
}

get_version() {
  if [[ -n "${VERSION:-}" ]]; then
    echo "$VERSION"
  else
    curl -fsSL "${GITHUB_API}/latest" \
      | grep -Po '"tag_name":\s*"\K([^"]+)'
  fi
}

clone_and_build() {
  local version="$1"
  log "Cloning ${REPO} (tag: ${version}) into ${TMPDIR}"
  git clone "https://github.com/${REPO}.git" "$TMPDIR/${BIN}"
  cd "$TMPDIR/${BIN}"

  if ! command -v cargo &>/dev/null; then
    log "error: cargo is required to build from source."
    exit 1
  fi

  log "Building ${BIN} with cargo"
  cargo build --release

  local built_bin="target/release/${BIN}"
  if [[ ! -x "$built_bin" ]]; then
    log "error: build failed, binary not found at ${built_bin}"
    exit 1
  fi

  echo "$PWD/$built_bin"
}

main() {
  log "Starting installation of ${BIN}"

  ensure_sudo

  local version
  version="$(get_version)"
  log "Resolved version: ${version}"

  local built_path
  built_path="$(clone_and_build "$version")"

  # Prompt if already installed
  if [[ -f "${INSTALL_DIR}/${BIN}" ]]; then
    read -p "${INSTALL_DIR}/${BIN} exists. Overwrite? [y/N]: " yn
    case "${yn,,}" in
      y|yes) ;;
      *) log "Aborted."; exit 0 ;;
    esac
  fi

  log "Installing to ${INSTALL_DIR}"
  $SUDO install -m 755 "$built_path" "${INSTALL_DIR}/${BIN}"

  log "Verifying installation..."
  if ! "${BIN}" --help &>/dev/null; then
    log "error: verification failed"
    exit 1
  fi

  log "Successfully installed ${BIN} ${version} to ${INSTALL_DIR}"
}

main

