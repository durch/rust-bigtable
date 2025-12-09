#!/bin/bash
# Creates a Bigtable instance, table, and column family for testing
set -e

PROJECT_ID="${PROJECT_ID:-gen-lang-client-0421059902}"
INSTANCE_ID="${INSTANCE_ID:-test-inst}"
CLUSTER_ID="${CLUSTER_ID:-test-cluster}"
ZONE="${ZONE:-us-central1-c}"
TABLE_NAME="${TABLE_NAME:-my-table}"
COLUMN_FAMILY="${COLUMN_FAMILY:-cf1}"

echo "Creating Bigtable instance..."
echo "  Project: $PROJECT_ID"
echo "  Instance: $INSTANCE_ID"
echo "  Cluster: $CLUSTER_ID"
echo "  Zone: $ZONE"

# Check if instance already exists
if gcloud bigtable instances describe "$INSTANCE_ID" --project="$PROJECT_ID" &>/dev/null; then
    echo "Instance '$INSTANCE_ID' already exists, skipping creation."
else
    gcloud bigtable instances create "$INSTANCE_ID" \
        --project="$PROJECT_ID" \
        --cluster="$CLUSTER_ID" \
        --cluster-zone="$ZONE" \
        --display-name="Test Instance"
    echo "Instance created."
fi

echo ""
echo "Creating table '$TABLE_NAME' with column family '$COLUMN_FAMILY'..."

# Check if table exists
if cbt -project="$PROJECT_ID" -instance="$INSTANCE_ID" ls 2>/dev/null | grep -q "^$TABLE_NAME$"; then
    echo "Table '$TABLE_NAME' already exists, skipping creation."
else
    cbt -project="$PROJECT_ID" -instance="$INSTANCE_ID" createtable "$TABLE_NAME"
    echo "Table created."
fi

# Check if column family exists
if cbt -project="$PROJECT_ID" -instance="$INSTANCE_ID" ls "$TABLE_NAME" 2>/dev/null | grep -q "$COLUMN_FAMILY"; then
    echo "Column family '$COLUMN_FAMILY' already exists, skipping creation."
else
    cbt -project="$PROJECT_ID" -instance="$INSTANCE_ID" createfamily "$TABLE_NAME" "$COLUMN_FAMILY"
    echo "Column family created."
fi

echo ""
echo "Bigtable setup complete!"
echo "  Instance: $INSTANCE_ID"
echo "  Table: $TABLE_NAME"
echo "  Column Family: $COLUMN_FAMILY"
