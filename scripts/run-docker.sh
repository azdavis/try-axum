#!/bin/sh
set -eu
# port matches with main.rs, name and tag match with build.sh
exec docker run -p 3000:3000 --init --rm --interactive --tty try-axum:latest
