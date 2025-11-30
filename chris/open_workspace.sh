#!/bin/bash
# Open the VS Code workspace for the given day (and create it if necessary)
# Usage: ./open_workspace.sh day01
set -euo pipefail
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
WORKSPACE_FILE="$SCRIPT_DIR/workspaces/$1.code-workspace"
if [ ! -e "$WORKSPACE_FILE" ]; then
# create workspace file
mkdir -p "$(dirname "$WORKSPACE_FILE")"
cat > "$WORKSPACE_FILE" << EOF
{
    "folders": [
        {
            "path": "../$1"
        }
    ]
}
EOF
fi
code -n "$WORKSPACE_FILE"
