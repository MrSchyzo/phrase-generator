FROM rust:1.58.1 as chef
WORKDIR /app
RUN apt update && apt install clang lld -y && cargo install sqlx-cli cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG DB_URL
ENV DATABASE_URL=$DB_URL
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN sqlx migrate revert --database-url $DB_URL && sqlx migrate run --database-url $DB_URL && cargo build --release --bin phrase-generator


FROM debian:bullseye-slim as runtime
WORKDIR app
COPY --from=builder /app/target/release/phrase-generator /usr/local/bin/app
USER 1000
CMD ["/usr/local/bin/app"]
