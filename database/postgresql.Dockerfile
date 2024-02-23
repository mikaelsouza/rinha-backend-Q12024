FROM postgres:16.2

ADD database/init.sh /docker-entrypoint-initdb.d/init-user-db.sh