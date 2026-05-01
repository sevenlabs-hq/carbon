#!/usr/bin/env bash
# Build + install frankfurt-carbon-shreds on the VPS.
#
# This script is idempotent and SAFE to run while the existing service is
# running — `cargo build --release` writes to a separate target dir, then
# `systemctl restart` swaps the binary. Frankfurt-rust is not touched.
#
# Run from the carbon-fork repo root:
#   sudo /opt/frankfurt-carbon-shreds/carbon/examples/frankfurt-carbon-shreds/deploy/deploy.sh
#
# Or from a deploy machine via SSH; assumes the VPS already has:
#   - /opt/frankfurt-carbon-shreds/carbon/  (a clone of luckyaxedev/carbon)
#   - /opt/frankfurt/env/frankfurt-carbon-shreds.env (populated)
#
# Exit non-zero if any step fails — we never proceed to restart unless the
# build succeeded.

set -euo pipefail

REPO_DIR="${REPO_DIR:-/opt/frankfurt-carbon-shreds/carbon}"
SERVICE="frankfurt-carbon-shreds"
ENV_FILE="/opt/frankfurt/env/${SERVICE}.env"
UNIT_FILE="/etc/systemd/system/${SERVICE}.service"

require_root() {
    if [[ $EUID -ne 0 ]]; then
        echo "ERROR: must run as root (systemctl + /etc/systemd writes)." >&2
        exit 1
    fi
}

require_env_file() {
    if [[ ! -f "$ENV_FILE" ]]; then
        echo "ERROR: env file missing at $ENV_FILE." >&2
        echo "Copy from examples/frankfurt-carbon-shreds/deploy/frankfurt-carbon-shreds.env.template," >&2
        echo "fill in values, chmod 600, chown root:root." >&2
        exit 1
    fi
    local mode
    mode=$(stat -c '%a' "$ENV_FILE")
    if [[ "$mode" != "600" ]]; then
        echo "WARN: env file mode is $mode; recommend 600 to protect SUPABASE_SERVICE_ROLE_KEY." >&2
    fi
}

ensure_unit_installed() {
    local src="${REPO_DIR}/examples/frankfurt-carbon-shreds/deploy/${SERVICE}.service"
    if [[ ! -f "$UNIT_FILE" ]] || ! cmp -s "$src" "$UNIT_FILE"; then
        echo "Installing systemd unit ${UNIT_FILE}"
        install -m 644 -o root -g root "$src" "$UNIT_FILE"
        systemctl daemon-reload
    fi
}

build_release() {
    echo "Building ${SERVICE} (release)..."
    pushd "$REPO_DIR" >/dev/null
    cargo build --release -p "${SERVICE}"
    popd >/dev/null
}

restart_or_start() {
    if systemctl is-active --quiet "$SERVICE"; then
        echo "Restarting ${SERVICE}..."
        systemctl restart "$SERVICE"
    else
        echo "Service ${SERVICE} is not active; not auto-starting."
        echo "When ready: systemctl enable --now ${SERVICE}"
    fi
}

main() {
    require_root
    require_env_file
    ensure_unit_installed
    build_release
    restart_or_start
    echo "Done. systemctl status ${SERVICE} | head -20"
}

main "$@"
