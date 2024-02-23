#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE DATABASE rinha;
    \c rinha
    CREATE TABLE "transactions" (
        "id" serial NOT NULL,
        PRIMARY KEY ("id"),
        "limit" bigint NOT NULL,
        "balance" bigint NOT NULL
    );
    INSERT INTO transactions ("limit", "balance") 
    VALUES (100000, 0),
       (80000, 0),
       (1000000, 0),
       (10000000, 0),
       (500000, 0);
EOSQL