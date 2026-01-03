#!/bin/bash

SRC_DIR="/usr/src/origin/"
DEST_DIR="/home/cargo/"

rsync -av --delete --force --ignore-errors \
    --exclude 'target' \
    "$SRC_DIR" "$DEST_DIR"

cargo run -p "${APP_PACKAGE}"