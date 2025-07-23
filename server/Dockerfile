# TODO: dont use alpine and move to slim

FROM rust:1.87-alpine AS builder
RUN apk add --no-cache musl-dev openssl openssl-dev pkgconfig git
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM alpine:3.17 AS runtime
RUN apk add --no-cache openssl ca-certificates
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/server ./server
COPY Rocket.toml ./
EXPOSE 4000

COPY entrypoint.sh /usr/src/server/entrypoint.sh
ENTRYPOINT ["/usr/src/app/entrypoint.sh"]
