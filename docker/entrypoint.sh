#!/bin/sh
set -e

DATA_DIR="${DATA_DIR:-/data}"
APP_USER="${APP_USER:-dash}"
APP_GROUP="${APP_GROUP:-dash}"

DB_PATH="${DB_PATH:-$DATA_DIR/dash.db}"
export DB_PATH

# Ensure data directory exists and is owned by the app user
mkdir -p "$DATA_DIR"
chown -R "$APP_USER:$APP_GROUP" "$DATA_DIR"

# If running as root, drop privileges to the app user 
if [ "$(id -u)" = "0" ]; then
  # Preserve DB_PATH (and potentially other env vars) when switching user
  exec su -s /bin/sh -c "DB_PATH=\"$DB_PATH\" exec /dash/server" "$APP_USER"
else
  exec /dash/server
fi
