#!/usr/bin/env python3
"""Call one Plannotator Inbox tool through its stdio MCP server.

Usage: inbox.py <tool> [arguments.json | -]

<tool> is an Inbox tool name, for example send_message or read_thread.
The arguments are a JSON object from a file, or from stdin with "-".
The script prints the tool result as JSON. Use it only when the session has
no `plannotator_inbox` tool.
"""
import json
import subprocess
import sys

PLANNOTATOR = "/Users/frondeus/.local/bin/plannotator"


def main() -> int:
    if len(sys.argv) not in (2, 3):
        print(__doc__, file=sys.stderr)
        return 2
    tool = sys.argv[1]
    source = sys.argv[2] if len(sys.argv) == 3 else None
    if source is None:
        arguments = {}
    elif source == "-":
        arguments = json.load(sys.stdin)
    else:
        with open(source, encoding="utf-8") as file:
            arguments = json.load(file)

    server = subprocess.Popen(
        [PLANNOTATOR, "inbox", "mcp"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        text=True,
    )

    def send(message):
        server.stdin.write(json.dumps(message) + "\n")
        server.stdin.flush()

    def response(request_id):
        for line in server.stdout:
            message = json.loads(line)
            if message.get("id") == request_id:
                return message
        raise RuntimeError("the Inbox server closed before it answered")

    try:
        send({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {
            "protocolVersion": "2025-06-18", "capabilities": {},
            "clientInfo": {"name": "kreatyna-inbox-script", "version": "1"}}})
        response(1)
        send({"jsonrpc": "2.0", "method": "notifications/initialized"})
        send({"jsonrpc": "2.0", "id": 2, "method": "tools/call",
              "params": {"name": tool, "arguments": arguments}})
        result = response(2)
    finally:
        server.stdin.close()
        server.terminate()

    if "error" in result:
        print(json.dumps(result["error"], indent=2), file=sys.stderr)
        return 1
    content = result["result"].get("content", [])
    text = "\n".join(item.get("text", "") for item in content if item.get("type") == "text")
    print(text)
    # Messages, answers, and annotations come as structured content.
    if "structuredContent" in result["result"]:
        print(json.dumps(result["result"]["structuredContent"], indent=2, ensure_ascii=False))
    return 1 if result["result"].get("isError") else 0


if __name__ == "__main__":
    sys.exit(main())
