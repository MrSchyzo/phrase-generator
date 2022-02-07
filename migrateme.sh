#!/bin/bash

if ! (hash cargo) ; then
  >&2 echo "Failed. You need cargo to run this."
  exit 1
fi

if ! (hash docker) ; then
  >&2 echo "Failed. You need docker to run this."
  exit 1
fi

docker stop dev-pgql >/dev/null 2>&1
docker run --name dev-pgql --rm -p 12345:5432 -e POSTGRES_PASSWORD=postgres -d postgres:14.1 && \
cargo install sqlx-cli && \
sleep 2 && \
sqlx migrate revert && \
sqlx migrate run
