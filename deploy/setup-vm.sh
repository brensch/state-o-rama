#!/bin/sh
# One-time (idempotent) setup of the brensch.com box for state-o-rama; `make setup-vm`
# runs it there as root after copying the unit and Caddy site to /tmp.
# Afterwards `make deploy` installs the binary.
set -eu
id stateorama >/dev/null 2>&1 || useradd --system --no-create-home --home-dir /nonexistent --shell /usr/sbin/nologin stateorama
install -d -m 755 /opt/state-o-rama
install -m 644 /tmp/state-o-rama.service /etc/systemd/system/state-o-rama.service
install -d -m 755 /etc/caddy/Caddyfile.d
install -m 644 /tmp/state-o-rama.caddyfile /etc/caddy/Caddyfile.d/state-o-rama.caddyfile
rm -f /tmp/state-o-rama.service /tmp/state-o-rama.caddyfile
out=$(caddy validate --config /etc/caddy/Caddyfile --adapter caddyfile 2>&1) || { echo "$out"; exit 1; }
systemctl reload caddy
systemctl daemon-reload
systemctl enable state-o-rama >/dev/null 2>&1
echo "state-o-rama set up; run make deploy"
