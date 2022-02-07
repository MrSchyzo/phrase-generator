## Intro

It requires:
- `lld` linker (`sudo apt install -y lld`)
- `cargo`
- `docker (19.03+)`
- `docker-compose`
- `jdk 11+`

This is a WIP, todo list:
- [ ] make it configurable
- [ ] make code more readable and more unit testable
- [ ] IP rate limiting (maybe possible through an nginx docker image as reverse proxy)

## Adding git hooks for this project

Run this command in the repository root
```shell script
git config --local core.hooksPath suggested_hooks
```

## Useful scripts

### `runme.sh`
It ensures that `phrasegen` runs in your localhost, read the output in order to locate all the useful URLs.

### `migrateme.sh`
It ensures that `phrasegen` runs its migrations in a dedicated dev postgres at `localhost` port `12345`.

### `buildme.sh`
It ensures that `phrasegen` runs its migrations and builds its executable.

## Run the example in Docker
### Noob version
```shell script
./runme.sh
```
That's all. `docker-compose down` in the repository root to stop the application.

### More "manual" version
Starting from the repository root
```shell script
cd ..
git clone git@github.com:MrSchyzo/tts-rest-wrapper.git
cd tts-rest-wrapper
./generate-replicas-local-deploy.sh
docker-compose up -d
cd ../phrase-generator
docker build -t phrase-generator .
docker run --rm -it -e TTS_WRAPPER_URL=http://nginx-lb:80 -p 8000:8000 --network tts-rest-wrapper_tts-network phrase-generator
```
It'll start:
- a docker-compose with a load-balanced tts-rest-wrapper connected to `tts-rest-wrapper_tts-network` docker network
- a docker container with the phrase-generator, reachable at `localhost:8000`

## Run without docker
You need:
- a running postgres
- `sqlx-cli` for the migrations
- a running `tts-rest-wrapper` instance
- `cargo` and `rustc`
- set `DB_CONNECTION_STRING` environment variable to the connection string for your running postgres instance
- set `TTS_WRAPPER_URL` environment variable to the running `tts-rest-wrapper` instance

What to do:
1. `sqlx migrate revert --database-url <DB_CONN_STRING> && sqlx migrate run --database-url <DB_CONN_STRING>`
1. `cargo build && cargo run`

## Schema explanation

TODO: write something about the schema in migrations.
