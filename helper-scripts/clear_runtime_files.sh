#!/bin/bash

PATHS_TO_CLEAR=(
	"../web-client/.nuxt"
	"../web-client/dist"
	"../web-client/.output"
)

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ERROR_OCCURRED=0

echo "🧾 The following paths will be cleared:"
TARGET_PATHS=()
for REL_PATH in "${PATHS_TO_CLEAR[@]}"; do
	ABS_PATH="$(realpath "$SCRIPT_DIR/$REL_PATH")"
	echo "  - $ABS_PATH"
	TARGET_PATHS+=("$ABS_PATH")
done

# Confirm with user
read -p "Are you sure you want to delete the contents of these directories? [y/N]: " CONFIRM
if [[ ! "$CONFIRM" =~ ^[Yy]$ ]]; then
	echo "Aborted."
	exit 0
fi

# Perform deletion
for DIR in "${TARGET_PATHS[@]}"; do
	if [ -d "$DIR" ]; then
		echo "🗑️  Clearing: $DIR"
		if rm -rf "$DIR"/*; then
			echo "Successfully cleared: $DIR"
		else
			echo "❌ ERROR: Failed to clear $DIR" >&2
			ERROR_OCCURRED=1
		fi
	else
		echo "Skipped (not found): $DIR"
	fi
done

# Final status
if [[ "$ERROR_OCCURRED" -eq 1 ]]; then
	echo "❌ One or more deletions failed."
	exit 1
else
	echo "All specified paths cleared successfully."
	exit 0
fi
