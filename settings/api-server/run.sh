#!/bin/bash

MYSQL_USER=$(cat /run/secrets/mysql_user) && export MYSQL_USER
MYSQL_PASSWORD=$(cat /run/secrets/mysql_password) && export MYSQL_PASSWORD

/app/api_server