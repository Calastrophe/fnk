#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

(trap 'kill 0' SIGINT; \
 bash -c 'cd frontend; dx serve' & \
 bash -c 'cd backend; cargo watch -- cargo run -- --port 8081')
