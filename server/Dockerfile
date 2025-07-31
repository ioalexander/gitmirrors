FROM rust:1.87-slim AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
  pkg-config libssl-dev libpq-dev ca-certificates postgresql-client curl git \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM rust:1.87-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
  build-essential bash libssl-dev libpq-dev postgresql-client openssh-client git curl ca-certificates \
  && rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --no-default-features --features postgres

ENV DATABASE_URL=postgres://postgres:postgres@localhost:5432/mydb

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/server /usr/src/app/server
COPY entrypoint.sh /usr/src/app/entrypoint.sh
COPY diesel.toml /usr/src/app/diesel.toml
COPY migrations /usr/src/app/migrations

RUN chmod +x /usr/src/app/entrypoint.sh

ENTRYPOINT ["/usr/src/app/entrypoint.sh"]
