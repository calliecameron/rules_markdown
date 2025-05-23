#!/usr/bin/env python3
"""Find git repos and related paths."""

import argparse
import json
import os
import os.path
import string
import unicodedata
from typing import Any

IGNORED = frozenset(
    [
        ".mypy_cache",
        ".vscode",
    ],
)


def normalised_char_name(char: str) -> str:
    if len(char) != 1:
        raise ValueError(f"char must be a single character, got '{char}'")
    valid = frozenset(string.ascii_uppercase + "_")
    return "".join([c for c in unicodedata.name(char) if c in valid])


def repo_name(path: str) -> str:
    # Names can only contain letters, numbers, dashes and underscores.
    path = path.replace("_", "__")
    valid = frozenset(string.ascii_letters + string.digits + "-_")
    parts = []
    for c in path:
        if c in valid:
            parts.append(c)
        else:
            parts.append("_" + normalised_char_name(c) + "_")
    return "git_repo_" + "".join(parts)


def walk(root: str) -> dict[str, Any]:
    packages: dict[str, str] = {}  # package -> repo name
    repos: dict[str, str] = {}  # repo name -> repo path relative to root
    dirs: set[str] = {""}  # dir paths relative to root

    def _walk_dir(path: str, parent_repo: str | None) -> None:
        subdirs = []
        repo = parent_repo
        package = ""
        with os.scandir(path) as entries:
            for entry in entries:
                if entry.name == "BUILD" and entry.is_file(follow_symlinks=False):
                    package = os.path.relpath(path, root)
                elif entry.name == ".git" and entry.is_dir(follow_symlinks=False):
                    repo = os.path.relpath(path, root) if path != root else ""
                elif entry.is_dir(follow_symlinks=False) and entry.name not in IGNORED:
                    subdirs.append(entry.path)

        if package and repo is not None:
            name = repo_name(repo)
            packages[package if package != "." else ""] = name
            repos[name] = repo

        for subdir in subdirs:
            dirs.add(os.path.relpath(subdir, root))
            _walk_dir(subdir, repo)

    _walk_dir(root, None)

    return {
        "packages": packages,
        "repos": repos,
        "dirs": sorted(dirs),
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("path")
    args = parser.parse_args()

    print(
        json.dumps(
            walk(args.path),
            sort_keys=True,
            indent=4,
        ),
    )


if __name__ == "__main__":
    main()
