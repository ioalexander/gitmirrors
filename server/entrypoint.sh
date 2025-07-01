#!/bin/bash
set -euo pipefail

echo "Running Diesel migrations..."

echo "Waiting for PostgreSQL at $POSTGRES_HOST:$POSTGRES_PORT..."

for i in {1..30}; do
	if pg_isready -h "$POSTGRES_HOST" -p "$POSTGRES_PORT" >/dev/null 2>&1; then
		echo "PostgreSQL is ready."
		break
	fi
	echo "PostgreSQL not ready yet... ($i)"
	sleep 1
done

if ! pg_isready -h "$POSTGRES_HOST" -p "$POSTGRES_PORT" >/dev/null 2>&1; then
	echo "Error: PostgreSQL is still not ready after 30 seconds."
	exit 1
fi

echo "Diesel setup..."
diesel setup

echo "Migrations list"
diesel migration list

echo "Running diesel migration..."
diesel migration run

echo "Starting app with ./server"
./server
