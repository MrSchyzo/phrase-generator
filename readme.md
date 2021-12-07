## Intro

It requires:
- `lld` linker (`sudo apt install -y lld`)
- `cargo`

This is a WIP

## Adding git hooks for this project

Run this command in the repository root
```shell script
git config --local core.hooksPath suggested_hooks
```

## Run the example in Docker
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

## TODO list
- `Resolver` must be a pluggable dependency, not an undefined amount of static functions
- pass the `AppCore` in as few places as possible
- external http requests must be placed in a dedicated module (eg. `bridge`)
- random phrase generator has to be implemented (now it's just a hardcoded struct...)
- beautify errors in GQL responses
- clean the messy code
