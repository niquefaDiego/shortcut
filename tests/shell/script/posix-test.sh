#!/bin/sh
BINARY="$(pwd)/target/debug/shortcut"
shortcut() {
    $BINARY "$@"
}

. ./src/shell/script/script-posix.sh
. ./tests/shell/script/run-test.sh
