#!/bin/sh

set -eu

cargo build
target/debug/try-axum 1> run-1.tmp 2> run-2.tmp &
run_pid="$!"

postgres -D db 1> postgres-1.tmp 2> postgres-2.tmp &
pg_pid="$!"

cleanup() {
  echo
  kill "$run_pid" "$pg_pid"
  rm *.tmp
}

trap cleanup EXIT
wait
