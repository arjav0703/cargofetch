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

# Ensure we can sudo (or are root)
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

# Figure out which version to pull
get_version() {
  if [[ -n "${VERSION:-}" ]]; then
    echo "$VERSION"
  else
    curl -fsSL "${GITHUB_API}/latest" \
      | grep -Po '"tag_name":\s*"\K([^"]+)' 
  fi
}

# Download the binary into TMPDIR
fetch_binary() {
  local version="$1"
  local url="https://github.com/${REPO}/releases/download/${version}/${BIN}"
  local out="${TMPDIR}/${BIN}"

  log "Downloading ${url}"
  if command -v curl &>/dev/null; then
    curl -fSL "$url" -o "$out"
  elif command -v wget &>/dev/null; then
    wget -qO "$out" "$url"
  else
    log "error: neither curl nor wget is available."
    exit 1
  fi

  chmod +x "$out"
  echo "$out"
}

main() {
  log "Starting installation of ${BIN}"

  local version
  version="$(get_version)"
  log "Resolved version: ${version}"

  local binpath
  binpath="$(fetch_binary "$version")"

  # Check existing installation
  if [[ -f "${INSTALL_DIR}/${BIN}" ]]; then
    read -p "${INSTALL_DIR}/${BIN} exists. Overwrite? [y/N]: " yn
    case "${yn,,}" in
      y|yes) ;;
      *) log "Aborted."; exit 0 ;;
    esac
  fi

  log "Installing to ${INSTALL_DIR}"
  $SUDO install -m755 "$binpath" "${INSTALL_DIR}/${BIN}"

  log "Verifying..."
  "${BIN}" --help &>/dev/null

  log "Installed ${BIN} ${version} to ${INSTALL_DIR}/${BIN}"
}

main "$@"

