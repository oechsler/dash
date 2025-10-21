#!/bin/sh
set -e

APP_USER="${APP_USER:-dash}"
APP_GROUP="${APP_GROUP:-dash}"

DATA_DIR="${DATA_DIR:-/data}"
mkdir -p "$DATA_DIR"
chown -R "$APP_USER:$APP_GROUP" "$DATA_DIR"

DB_PATH="${DB_PATH:-$DATA_DIR/dash.db}"
export DB_PATH

# If running as root, drop privileges to the app user
if [ "$(id -u)" = "0" ]; then
  exec su -s /bin/sh -c "exec /dash/server" "$APP_USER"
else
  exec /dash/server
fi