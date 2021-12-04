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

## TODO list
- `Resolver` must be a pluggable dependency, not an undefined amount of static functions
- pass the `AppCore` in as few places as possible
- external http requests must be placed in a dedicated module (eg. `bridge`)
- random phrase generator has to be implemented (now it's just an hardcoded struct...)