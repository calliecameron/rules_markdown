#!/usr/bin/env python3
"""Get the version of a git repo."""

import argparse
import datetime
import os
import os.path
import re
import subprocess
from dataclasses import dataclass


@dataclass(frozen=True)
class GitCommit:
    commit: str
    dirty: bool
    timestamp: int

    def __str__(self) -> str:
        timestamp = datetime.datetime.fromtimestamp(self.timestamp, tz=datetime.UTC)
        return f"{self.commit}{'-dirty' if self.dirty else ''}, {timestamp}"


def get_git_commit(path: str) -> GitCommit | None:
    os.chdir(path)
    try:
        subprocess.run(
            ["git", "rev-parse", "--git-dir"],
            capture_output=True,
            encoding="utf-8",
            check=True,
        )
    except subprocess.CalledProcessError:
        # Not a git repo
        return None

    try:
        commit = subprocess.run(
            ["git", "describe", "--no-match", "--always", "--long"],
            capture_output=True,
            encoding="utf-8",
            check=True,
        ).stdout.strip()
    except subprocess.CalledProcessError:
        # New repo with no commits yet
        return None

    try:
        timestamp = int(
            subprocess.run(
                ["git", "show", "--no-patch", "--no-notes", "--pretty=%ct", commit],
                capture_output=True,
                encoding="utf-8",
                check=True,
            ).stdout.strip(),
        )

        status = subprocess.run(
            ["git", "status", "--porcelain", "-b"],
            capture_output=True,
            encoding="utf-8",
            check=True,
        ).stdout.strip()
        dirty = False
        for line in status.split("\n"):
            if re.fullmatch(r"##.*\[ahead.*\]", line) is not None or not line.startswith("##"):
                dirty = True
                break

    except subprocess.CalledProcessError:
        return None

    return GitCommit(commit, dirty, timestamp)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("path")
    args = parser.parse_args()

    commit = get_git_commit(args.path)

    print(str(commit) if commit else "unversioned")


if __name__ == "__main__":
    main()
