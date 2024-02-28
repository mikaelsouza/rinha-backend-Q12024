#!/bin/bash
set -e
# TODO: Change timezone if necessary
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    SET TIMEZONE='America/Manaus';
    CREATE DATABASE rinha;
    \c rinha
    CREATE TABLE "accounts" (
        "id" bigserial NOT NULL,
        "limit" bigint NOT NULL,
        "balance" bigint NOT NULL,
        PRIMARY KEY ("id")
    );
    INSERT INTO accounts ("limit", "balance")
    VALUES (100000, 0),
       (80000, 0),
       (1000000, 0),
       (10000000, 0),
       (500000, 0);
    CREATE TYPE transaction_type as ENUM ('C', 'D');
    CREATE TABLE "transactions" (
        "id" bigserial NOT NULL,
        "account_id" bigserial NOT NULL,
        "value" bigint NOT NULL,
        "transaction_type" transaction_type NOT NULL,
        "description" varchar(10) NOT NULL,
        "timestamp" timestamptz NOT NULL DEFAULT NOW(),
        PRIMARY KEY ("id"),
        FOREIGN KEY("account_id") REFERENCES accounts(id)
    );
    INSERT INTO "transactions" ("account_id", "value", "transaction_type", "description", "timestamp")
    VALUES (1, '10000', 'C', 'sdadsa', now()),
           (1, '10000', 'D', 'sdadsa', now()),
           (2, '10000', 'C', 'sdadsa', now());
EOSQL