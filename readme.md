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

## How to use

Run `runme.sh`, it'll expose a GraphQL endpoint, as stated by the script's output.
Then, use the example dataset contained in `example-dump.zip` in order to have an already functioning generator.

### GraphQL query to send

You can access the playground at `http://localhost:81` for a web GUI exposed by this application.
The query is as follows.

```graphql
query {
  random(opts: {category: ""}) {
    id
    text
    audioUrl(voice:{language:ITA,gender:MALE})
  }
}
```

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

## How does the generation work

~~Badly~~ All the generation depends on the data stored in its PostgreSQL.

The main concepts are three:
- Structure generation through a BNF-ish format: see tables `production` and `non_terminal_symbol`
- Word choice through relationships, grammar and semantic propagation; see those tables:
    - `grammar_tag`
    - `semantic_tag`
    - `word`
    - `word_grammar_compatibility`
    - `word_grammar_requirements`
    - `word_semantic`
- tts conversion, using an instance of [`tts-rest-wrapper`](https://github.com/MrSchyzo/tts-rest-wrapper), see tables:
    - `generated_phrase`
    - `generated_phrase_speech`
    
### Structure generation

All the generation can be seen as a glorified BNF grammar generation:
every non-terminal symbol has a set of related productions which in turn potentially expand to other
non-terminal symbol.

Non-terminal symbols are just symbolic names used to narrow down the possible productions.
So far, `phrasegen` assumes `Start` non-terminal symbol always exists and has at least an associated production.

Productions are strings that can only contain these two kinds of "selectors":
- `<ID:GDepends:GPropagate:SDepends:SPropagate:Semtags>`, a word selector
- `{ID:GDepends:GPropagate:SDepends:SPropagate:NonTerminalSymbol}`, a production selector

These two selectors share a common 80% of their structure.

#### Selector common structure
- `ID` identifying number of the selector inside a single production
- `GDepends` grammar dependency, used to select words accordingly, its value can be:
    - `N`: completely independent
    - `C`: depends on context (it'll be explained later) only
    - `O(other)`: depends on selector with ID `other`
    - `OC(other)`: `C` AND `O(other)`
- `GPropagate` whether it propagates its resulting grammar, can be `T` (true) or `F` (false)
- `SDepends` semantic dependency, analogous to grammar dependency
- `SPropagate` analogous to grammar propagation

#### Word selector `Semtags`
A comma-separated string of semantic tags to be satisfied when selecting a word.

#### Production selector `NonTerminalSymbol`
It is a string that identifies a non-terminal symbol for production expansion.

### TTS conversion

This part triggers only if requesting the `audioUrl` field.
In that case, `phrasegen` asks `tts-rest-wrapper` to generate a speech from a text.
In order to avoid spamming `tts-rest-wrapper`:
- A new phrase is generated with a chance of `(1 - currentGenerated / maximumGenerated)`
- Whenever a request has been fulfilled, the result is stored into a table in order to ask `tts-rest-wrapper` 
  only once for the same request
  
### Word selection
Words have two kinds of tags: semantics and grammar. Those tags dictate how to do a word selection.

#### Grammar vs semantics
Grammar and semantics are defined similarly, but behave differently in regard to propagation and dependency.

TODO explain:
- grammar: what are those, requirements vs output
- semantics: what are those, sticky vs non-sticky
- words: non-repeatable vs repeatable
- propagation and dependency in regard to production selector
- propagation and dependency in regard to word selector
