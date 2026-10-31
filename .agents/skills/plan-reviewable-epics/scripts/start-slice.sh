#!/usr/bin/env bash
# Start a slice: claim its bead and attach the mol-slice workflow.
#
#   start-slice.sh <slice-bead> <slice-id> <epic-folder>
#   start-slice.sh vibetyna-4rn.11 MAC-06 mac-shell
#
# Beads names the workflow root after the formula and both gates "Gate: human".
# This script renames them, so `bd ready` and `bd gate list` read clearly.
set -euo pipefail

if [ $# -ne 3 ]; then
  echo "usage: $0 <slice-bead> <slice-id> <epic-folder>" >&2
  exit 2
fi
bead=$1 slice=$2 epic=$3

bd update "$bead" --claim >/dev/null
bd mol bond mol-slice "$bead" --type parallel --var "slice=$slice" --var "epic=$epic" >/dev/null

root=$(bd list --parent "$bead" --title mol-slice --json | python3 -I -c 'import json, sys; print(json.load(sys.stdin)[0]["id"])')
bd update "$root" --title "$slice workflow" >/dev/null

# Each gate's description names the step it blocks: implement waits for the
# plan, finish for the approval.
bd list --parent "$root" --type gate --json | python3 -I -c '
import json, sys
for gate in json.load(sys.stdin):
    print(gate["id"], gate["description"].rsplit(" ", 1)[-1])
' | while read -r gate step; do
  case "$step" in
    implement) bd update "$gate" --title "$slice: the user accepts the plan" >/dev/null ;;
    finish) bd update "$gate" --title "$slice: the user approves the slice" >/dev/null ;;
    *) echo "unexpected gate $gate for step $step" >&2; exit 1 ;;
  esac
done

bd graph "$bead" --compact
