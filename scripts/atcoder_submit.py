#!/usr/bin/env python3

import argparse
import html
import json
import os
import re
import sys
from html.parser import HTMLParser
from pathlib import Path

import requests


class SubmitPageParser(HTMLParser):
    def __init__(self):
        super().__init__()
        self.csrf_token = None
        self.in_option = False
        self.option_value = None
        self.option_text = []
        self.languages = []

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if tag == "input" and attrs.get("name") == "csrf_token":
            self.csrf_token = attrs.get("value")
        elif tag == "option" and "value" in attrs:
            self.in_option = True
            self.option_value = attrs["value"]
            self.option_text = []

    def handle_data(self, data):
        if self.in_option:
            self.option_text.append(data)

    def handle_endtag(self, tag):
        if tag == "option" and self.in_option:
            label = html.unescape("".join(self.option_text)).strip()
            self.languages.append((self.option_value, label))
            self.in_option = False
            self.option_value = None
            self.option_text = []


def load_acc_cookies(session):
    path = Path.home() / ".config/atcoder-cli-nodejs/session.json"
    if not path.exists():
        raise RuntimeError("acc session not found. Run `acc login` first.")

    data = json.loads(path.read_text())
    for item in data.get("cookies", []):
        if "=" not in item:
            continue
        name, value = item.split("=", 1)
        session.cookies.set(name, value, domain="atcoder.jp")


def find_task(contest_dir, task_dir):
    config_path = contest_dir / "contest.acc.json"
    if not config_path.exists():
        raise RuntimeError(f"contest.acc.json not found: {config_path}")

    config = json.loads(config_path.read_text())
    rel_task_dir = task_dir.relative_to(contest_dir).as_posix()
    for task in config["tasks"]:
        if task["directory"]["path"] == rel_task_dir:
            return config["contest"]["id"], task["id"], task["url"]

    raise RuntimeError(f"task not found in contest.acc.json: {rel_task_dir}")


def choose_language(languages):
    language_id = os.environ.get("ATCODER_LANGUAGE_ID")
    if language_id:
        for value, label in languages:
            if value == language_id:
                return value, label
        return language_id, f"language id {language_id}"

    preferred_patterns = [
        r"C\+\+23 .*GCC",
        r"GNU\+\+20",
        r"C\+\+.*GCC",
    ]
    for pattern in preferred_patterns:
        for value, label in languages:
            if re.search(pattern, label):
                return value, label

    cpp_languages = [(value, label) for value, label in languages if "C++" in label]
    if cpp_languages:
        return cpp_languages[0]

    raise RuntimeError("C++ language was not found on the submit page.")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("source")
    parser.add_argument("--yes", "-y", action="store_true")
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()

    src = Path(args.source).resolve()
    task_dir = src.parent
    contest_dir = task_dir.parent
    contest_id, task_id, task_url = find_task(contest_dir, task_dir)
    code = src.read_text()

    session = requests.Session()
    load_acc_cookies(session)

    submit_url = f"https://atcoder.jp/contests/{contest_id}/submit"
    response = session.get(submit_url, timeout=20)
    response.raise_for_status()

    page = SubmitPageParser()
    page.feed(response.text)
    if not page.csrf_token:
        raise RuntimeError("csrf_token was not found. Are you logged in?")

    language_id, language_label = choose_language(page.languages)

    print(f"submit to: {task_url}")
    print(f"source: {src}")
    print(f"language: {language_label} ({language_id})")

    if args.dry_run:
        print("dry-run: not submitted")
        return 0

    if not args.yes and sys.stdin.isatty():
        answer = input("Submit? [y/N] ").strip().lower()
        if answer not in ("y", "yes"):
            print("cancelled")
            return 1

    payload = {
        "data.TaskScreenName": task_id,
        "data.LanguageId": language_id,
        "sourceCode": code,
        "csrf_token": page.csrf_token,
    }
    result = session.post(submit_url, data=payload, timeout=20, allow_redirects=False)
    if result.status_code not in (302, 303):
        print(f"submit failed: HTTP {result.status_code}", file=sys.stderr)
        print(result.text[:1000], file=sys.stderr)
        return 1

    print("submitted")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
