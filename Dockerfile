FROM rust:1.57.0 as chef
WORKDIR /app
RUN cargo install cargo-chef && apt update && apt install clang lld -y

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin phrase-generator


FROM debian:bullseye-slim as runtime
WORKDIR app
COPY --from=builder /app/target/release/phrase-generator /usr/local/bin/app
USER 1000
CMD ["/usr/local/bin/app"]
