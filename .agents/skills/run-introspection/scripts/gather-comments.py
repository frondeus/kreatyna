"""Gather the user's Plannotator comments on this repository since a time.

usage: gather-comments.py --since ISO_TIME --out DIR [--repo PATH]
       gather-comments.py --since 1970-01-01T00:00:00Z --out DIR   (all)

The script only reads. It writes four Markdown files into DIR:

  inbox.md         every message the user wrote in the Inbox, in full, by
                   thread; each agent message as its first line, for context
  annotations.md   comments on attached files and images
  annotate.md      `plannotator annotate` feedback and plan-review feedback
                   from before the Inbox (history/ and plans/)
  code-reviews.md  `plannotator review` feedback that reached Claude Code
                   sessions (from the session transcripts)

It prints the newest comment time it saw. That is the next cursor.
"""
import argparse
import collections
import glob
import json
import os
import pathlib
import re
import sys

HOME = pathlib.Path.home()
PLANNOTATOR = HOME / ".plannotator"


def args():
    p = argparse.ArgumentParser()
    p.add_argument("--since", required=True, help="ISO time, for example 2026-11-04T22:35:00Z")
    p.add_argument("--out", required=True)
    p.add_argument("--repo", default=os.getcwd())
    return p.parse_args()


def project_dir(repo):
    """The Inbox project folder whose root is this repository."""
    for project in glob.glob(str(PLANNOTATOR / "inbox/projects/*/project.json")):
        data = json.load(open(project))
        if os.path.realpath(data.get("root", "")) == os.path.realpath(repo):
            return pathlib.Path(project).parent
    sys.exit(f"no Plannotator Inbox project has the root {repo}")


def latest(path):
    """The last version of each record in a .jsonl log, by id."""
    records = {}
    if path.exists():
        for line in open(path):
            if line.strip():
                row = json.loads(line)
                records[row["id"]] = row["record"]
    return records


def inbox(folder, since, newest):
    messages = latest(folder / "messages.jsonl")
    threads = collections.defaultdict(list)
    for m in messages.values():
        threads[m["thread_id"]].append(m)
    out = []
    for _, ms in sorted(threads.items(), key=lambda kv: min(m["created_at"] for m in kv[1])):
        ms.sort(key=lambda m: m["created_at"])
        person = [m for m in ms if m["author"]["kind"] == "person" and m["created_at"] > since]
        if not person:
            continue
        out.append(f"\n\n######## THREAD {ms[0]['thread_id']}: {ms[0].get('subject')}\n")
        for m in ms:
            body = m.get("body") or ""
            if m["author"]["kind"] == "person":
                if m["created_at"] <= since:
                    continue
                newest.append(m["created_at"])
                out.append(f"\n--- USER {m['created_at']} ({m['id']})\n{body}\n")
            elif m["created_at"] > since:
                first = body.strip().split("\n")[0][:200]
                out.append(f"\n--- agent {m['created_at']} ({m['id']}): {first}\n")
    return "".join(out)


def annotations(folder, since, newest):
    out = []
    rows = latest(folder / "annotations.jsonl").values()
    for a in sorted(rows, key=lambda a: a.get("created_at") or ""):
        if (a.get("created_at") or "") <= since or a.get("removed_at"):
            continue
        newest.append(a["created_at"])
        note = a["annotation"]
        out.append(
            f"\n--- {a['created_at']} {a.get('path')} ({note.get('type')})\n"
            f"QUOTE: {(note.get('originalText') or '')[:300]}\n"
            f"COMMENT: {note.get('text', '')}\n"
        )
    return "".join(out)


def annotate(repo, since, newest):
    """History submissions and plan-review annotations about this repository."""
    out = []
    for path in sorted(glob.glob(str(PLANNOTATOR / "history/*/*/submissions/*.md"))):
        text = open(path).read()
        source = re.search(r"^- Source: (.+)$", text, re.M)
        submitted = re.search(r"^- Submitted: (.+)$", text, re.M)
        if not source or not submitted or not source.group(1).startswith(repo):
            continue
        if submitted.group(1) <= since or "has no feedback" in text:
            continue
        newest.append(submitted.group(1))
        out.append(f"\n######## {submitted.group(1)} {source.group(1)}\n{text}\n")
    return "".join(out)


def code_reviews(repo, since, newest):
    """`# Code Review Feedback` blocks from this repository's Claude Code sessions."""
    folder = HOME / ".claude/projects" / re.sub(r"[^A-Za-z0-9]", "-", repo)
    out, seen = [], set()
    for path in sorted(glob.glob(str(folder / "*.jsonl")), key=os.path.getmtime):
        for line in open(path):
            if "# Code Review Feedback" not in line:
                continue
            row = json.loads(line)
            stamp = row.get("timestamp", "")
            if row.get("type") != "user" or stamp <= since:
                continue
            content = row["message"].get("content")
            items = content if isinstance(content, list) else [{"text": content}]
            for item in items:
                if not isinstance(item, dict):
                    continue
                part = item.get("content", item.get("text"))
                text = part if isinstance(part, str) else "\n".join(
                    x.get("text", "") for x in (part or []) if isinstance(x, dict))
                # Skip the agent's own reads of earlier gathered files.
                if "# Code Review Feedback\n\n**Diff:**" not in text or "######## " in text:
                    continue
                body = text[text.index("# Code Review Feedback"):].split("Treat the findings above")[0]
                if body in seen:
                    continue
                seen.add(body)
                newest.append(stamp)
                out.append(f"\n######## {stamp}\n{body}\n")
    return "".join(out)


def main():
    a = args()
    repo = os.path.realpath(a.repo)
    out = pathlib.Path(a.out)
    out.mkdir(parents=True, exist_ok=True)
    newest = []
    files = {
        "inbox.md": inbox(project_dir(repo), a.since, newest),
        "annotations.md": annotations(project_dir(repo), a.since, newest),
        "annotate.md": annotate(repo, a.since, newest),
        "code-reviews.md": code_reviews(repo, a.since, newest),
    }
    for name, text in files.items():
        (out / name).write_text(text)
        print(f"{out / name}: {len(text)} characters")
    print("newest comment:", max(newest) if newest else "none")


main()
