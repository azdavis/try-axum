#!/bin/sh
set -eux
exec docker run --init --rm --interactive --tty try-axum:latest
