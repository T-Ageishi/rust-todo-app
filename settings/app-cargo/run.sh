#!/bin/bash

SRC_DIR="/usr/src/origin/"
DEST_DIR="/home/cargo/"

rsync -av --delete --force --ignore-errors \
    --exclude 'target' \
    "$SRC_DIR" "$DEST_DIR"

# TODO: 別の方法を使う
MYSQL_USER=$(cat /run/secrets/mysql_user) && export MYSQL_USER
MYSQL_PASSWORD=$(cat /run/secrets/mysql_password) && export MYSQL_PASSWORD

cargo run