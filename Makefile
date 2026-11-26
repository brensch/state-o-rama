# Build and ship state-o-rama to the brensch.com box (x86_64 Debian, the one
# that also runs snekgame and ppp; SSH goes through IAP via the schlift-gcp alias).
#   make build     static (musl) release binary, so the box's glibc doesn't matter
#   make deploy    build, copy to the box, restart the service
#   make setup-vm  one-time box setup (user, systemd unit, Caddy site)
#   make shapes    regenerate shapes/ and web/*.json from the map data
VM ?= schlift-gcp
TARGET = x86_64-unknown-linux-musl
BIN = target/$(TARGET)/release/state-o-rama

.PHONY: build deploy setup-vm shapes test
build:
	cargo build --release --target $(TARGET)

test:
	cargo test

deploy: test build
	scp $(BIN) $(VM):/tmp/state-o-rama
	ssh $(VM) 'sudo install -m 755 /tmp/state-o-rama /opt/state-o-rama/state-o-rama && rm /tmp/state-o-rama && sudo systemctl restart state-o-rama && sleep 1 && systemctl is-active state-o-rama'

setup-vm:
	scp deploy/state-o-rama.service deploy/state-o-rama.caddyfile $(VM):/tmp/
	ssh $(VM) 'sudo sh -s' < deploy/setup-vm.sh

shapes:
	cd tools && npm ci && mkdir -p data && \
	  ([ -f data/ne_admin1.geojson ] || curl -fsSL -o data/ne_admin1.geojson https://raw.githubusercontent.com/nvkelso/natural-earth-vector/master/geojson/ne_10m_admin_1_states_provinces.geojson) && \
	  node gen-shapes.mjs
