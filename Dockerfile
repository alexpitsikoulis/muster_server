FROM lukemathwalker/cargo-chef:latest-rust-slim-bullseye as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path=recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
RUN  cargo build --release --bin muttr_server

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update - \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/muttr_server muttr_server
COPY config config
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./muttr_server"]