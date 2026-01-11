#!/bin/sh
BINARY="$(pwd)/target/debug/shortcut"
shortcut() {
    $BINARY "$@"
}

. ./src/shell/script/posix.sh
. ./tests/shell/script/run-test.sh
