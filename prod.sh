#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

(trap 'kill 0' SIGINT; \
 bash -c 'cd frontend; dx serve --release' & \
 bash -c 'cd backend; cargo run --bin backend --release -- --port 8080 --static-dir ./dist')
