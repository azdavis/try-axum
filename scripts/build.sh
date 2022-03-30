#!/bin/sh
set -eux
exec docker build --tag try-axum:latest .
