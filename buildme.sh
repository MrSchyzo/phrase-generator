#!/bin/bash

./migrateme.sh && \
cargo clippy && \
cargo build
