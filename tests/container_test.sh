#!/bin/bash
# Runs tests in a container, mainly to detect non-hermetic dependencies.

set -eu

THIS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

"${THIS_DIR}/container_build.sh"

podman run \
    --rm \
    -it \
    --user "$(id -u):$(id -g)" \
    -e "TERM=${TERM}" \
    rules-markdown-container-test \
    -c './run_tests.sh'
