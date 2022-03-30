#!/bin/sh
set -eu
export DOCKER_SCAN_SUGGEST=false
exec docker build --tag try-axum:latest .
