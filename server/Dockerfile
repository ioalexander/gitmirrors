FROM rust:1.87-slim as builder

RUN apt-get update && apt-get install -y --no-install-recommends \
  pkg-config libssl-dev libpq-dev ca-certificates postgresql-client curl git \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -r src

COPY . .
RUN cargo build --release

FROM rust:1.87-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
  build-essentials bash libssl-dev libpq-dev postgresql-client git openssh-client curl ca-certificates \
  && rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/server /usr/src/app/server
COPY entrypoint.sh /usr/src/app/entrypoint.sh
RUN chmod +x /usr/src/app/entrypoint.sh

ENTRYPOINT ["/usr/src/app/entrypoint.sh"]
