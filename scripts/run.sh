#!/bin/sh
set -eu
exec docker run --init --rm --interactive --tty try-axum:latest
