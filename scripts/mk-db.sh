#!/bin/sh

set -eu

db_name="db"

echo "removing '$db_name' if it exists"
rm -rf "$db_name"

echo "initializing '$db_name'"
initdb "$db_name" 1> initdb-1.tmp 2> initdb-2.tmp

echo "starting postgres for '$db_name'"
postgres -D "$db_name" 1> postgres-1.tmp 2> postgres-2.tmp &
pg_pid="$!"

superuser="postgres"
try=1
max=5
while true; do
  echo "creating superuser '$superuser' on '$db_name'"
  sleep 1
  if createuser -s "$superuser"; then
    break
  fi
  try=$((try + 1))
  if [ "$try" -ge "$max" ]; then
    echo "error: too many tries"
    exit 1
  fi
  echo "didn't work, trying again"
done

echo "stopping postgres"
kill "$pg_pid"

echo "removing temp logs"
rm *.tmp
