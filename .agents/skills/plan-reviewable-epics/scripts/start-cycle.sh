#!/usr/bin/env bash
# Start a delivery cycle: pour the mol-cycle workflow (4 feature slices, the
# open bugs, the full test suite, the cooldown, the introspection, and the
# user's approval).
#
#   start-cycle.sh <cycle>
#   start-cycle.sh 2
#
# Beads names the workflow root after the formula and the gate "Gate: human".
# This script renames them, so `bd ready` and `bd gate list` read clearly.
set -euo pipefail

if [ $# -ne 1 ]; then
  echo "usage: $0 <cycle>" >&2
  exit 2
fi
cycle=$1

bd mol pour mol-cycle --var "cycle=$cycle" >/dev/null

root=$(bd list --type molecule --title mol-cycle --status open --json | python3 -I -c '
import json, sys
roots = json.load(sys.stdin)
if len(roots) != 1:
    sys.exit(f"expected one new mol-cycle root, found {len(roots)}")
print(roots[0]["id"])
')
bd update "$root" --title "Cycle $cycle" >/dev/null

gate=$(bd list --parent "$root" --type gate --json | python3 -I -c '
import json, sys
gates = json.load(sys.stdin)
if len(gates) != 1:
    sys.exit(f"expected one gate, found {len(gates)}")
print(gates[0]["id"])
')
bd update "$gate" --title "Cycle $cycle: the user approves the cooldown and the introspection" >/dev/null

bd graph "$root" --compact
