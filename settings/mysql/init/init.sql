-- Create database
CREATE DATABASE IF NOT EXISTS app_db CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_as_cs;
USE app_db;

-- Create role
CREATE ROLE IF NOT EXISTS `app_user_role`;
GRANT
    SELECT,
        INSERT,
        UPDATE,
        DELETE
        ON app_db.*
    TO 'app_user_role';

-- Crate user
GRANT 'app_user_role' TO 'app_user';

-- Create tables
CREATE TABLE IF NOT EXISTS tasks
(
    id          VARCHAR(128) NOT NULL PRIMARY KEY,
    title       VARCHAR(64)  NOT NULL,
    description VARCHAR(256) NOT NULL,
    status      INT          NOT NULL
);