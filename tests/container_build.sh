#!/bin/bash
# Build a container for testing.

set -eu

THIS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${THIS_DIR}/container"

NIXPKGS_COMMIT="$(tr '\n' '\t' <../../MODULE.bazel |
    grep -o -P 'nix_repo\.github\(.*?\)' |
    tr '\t' '\n' |
    grep commit |
    grep -o '".*"' |
    sed 's/"//g')"

BAZEL_VERSION="$(grep USE_BAZEL_VERSION ../../.bazeliskrc | sed 's/^.*=//g')"

podman build \
    -f Containerfile \
    --build-arg "uid=$(id -u)" \
    --build-arg "gid=$(id -g)" \
    --build-arg "nixpkgs_commit=${NIXPKGS_COMMIT}" \
    --build-arg "bazel_version=${BAZEL_VERSION}" \
    -t rules-markdown-container-test \
    ../../
