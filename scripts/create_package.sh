#!/usr/bin/env bash

ROOT_DIR=$(dirname $(dirname $(readlink -f $0)))
TARGET=target
PACKAGE_FILE=$TARGET/rusty.zip
ZIP=zip

if ! hash $ZIP 2>/dev/null; then
    echo "zip not found"
    exit
fi

cd $ROOT_DIR

if [ ! -d $TARGET ]; then
    mkdir $TARGET
fi

zip $PACKAGE_FILE Cargo.lock Cargo.toml -r src
