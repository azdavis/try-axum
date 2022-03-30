#!/bin/sh
set -eu
docker image prune -f
export DOCKER_SCAN_SUGGEST=false
exec docker build --tag try-axum:latest .
