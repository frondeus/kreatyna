#!/usr/bin/env bash
# Draw one epic's slice graph from beads as a static image (no scripts, works offline).
#
#   epic-graph.sh <epic-bead> <out.svg|out.png>
#   epic-graph.sh vibetyna-hrx target/beads/recipes-graph.png
#
# `bd graph --html` loads D3 from the internet and draws nothing in the Inbox,
# and `bd graph --dot` also pulls in linked epics and parent edges. This keeps
# the epic's slices and their dependency edges only; slices of another epic
# that they depend on appear dashed.
set -euo pipefail

if [ $# -ne 2 ]; then
  echo "usage: $0 <epic-bead> <out.svg|out.png>" >&2
  exit 2
fi
epic=$1 out=$2
format=${out##*.}

bd graph --dot "$epic" | python3 -I -c '
import re, sys
epic = sys.argv[1]
src = sys.stdin.read()
nodes = dict(re.findall(r"^\s+(\"[^\"]+\") (\[label=.*\]);$", src, re.M))
edges = re.findall(r"^\s+(\"[^\"]+\") -> (\"[^\"]+\") \[style=solid[^\]]*\];$", src, re.M)
mine = lambda n: n.strip("\"").startswith(epic + ".")
keep = [(a, b) for a, b in edges if mine(a) or mine(b)]
used = {n for e in keep for n in e} | {n for n in nodes if mine(n)}
print("digraph epic {\n  rankdir=LR;")
print("  node [shape=box, style=\"rounded,filled\", fontname=\"Helvetica\", fontsize=11];")
print("  edge [color=\"#666666\"];")
for n in sorted(used):
    label = nodes[n] if mine(n) else nodes[n].replace("fillcolor=\"", "style=\"rounded,dashed\", fillcolor=\"", 1)
    print(f"  {n} {label};")
for a, b in keep:
    print(f"  {a} -> {b};")
print("}")
' "$epic" | nix shell nixpkgs#graphviz -c dot "-T$format" -Gdpi=110 -o "$out"
echo "$out"
