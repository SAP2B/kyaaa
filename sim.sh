#!/usr/bin/env bash
set -euo pipefail

CURRENT_SCRIPT=$(basename "$0")
REPORT_FILE="report.md"

echo "# Execution Report" > "$REPORT_FILE"
echo "Generated at: $(date)" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

for script in *.sh; do
    if [ -f "$script" ] && [ "$script" != "$CURRENT_SCRIPT" ]; then
        chmod +x "$script"
        
        echo "## Running: \`$script\`" >> "$REPORT_FILE"
        echo '```bash' >> "$REPORT_FILE"
        
        ./"$script" 2>&1 | sed 's/\x1B\[[0-9;]*[mK]//g' >> "$REPORT_FILE" || true
        
        echo '```' >> "$REPORT_FILE"
        echo "" >> "$REPORT_FILE"
    fi
done

echo "Report generated successfully: $REPORT_FILE"
