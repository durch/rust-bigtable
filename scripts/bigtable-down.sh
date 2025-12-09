#!/bin/bash
# Deletes the Bigtable instance (and all tables within it)
set -e

PROJECT_ID="${PROJECT_ID:-gen-lang-client-0421059902}"
INSTANCE_ID="${INSTANCE_ID:-test-inst}"

echo "Deleting Bigtable instance..."
echo "  Project: $PROJECT_ID"
echo "  Instance: $INSTANCE_ID"
echo ""

# Check if instance exists
if ! gcloud bigtable instances describe "$INSTANCE_ID" --project="$PROJECT_ID" &>/dev/null; then
    echo "Instance '$INSTANCE_ID' does not exist, nothing to delete."
    exit 0
fi

read -p "Are you sure you want to delete instance '$INSTANCE_ID'? This will delete ALL tables. [y/N] " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    gcloud bigtable instances delete "$INSTANCE_ID" --project="$PROJECT_ID" --quiet
    echo "Instance '$INSTANCE_ID' deleted."
else
    echo "Aborted."
    exit 1
fi
