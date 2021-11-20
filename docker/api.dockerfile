FROM rust:1.56.1-alpine AS build

RUN apk add --no-cache \
        openssl-dev musl-dev pkgconfig
ENV RUSTFLAGS="-C target-feature=-crt-static"

WORKDIR /src
COPY . .
RUN cargo --version && rustc --version && \
    cargo build --release --bin api

FROM alpine

RUN apk add --no-cache libgcc

WORKDIR /app
COPY --from=build /src/target/release/api /app/app

ENTRYPOINT ["/app/app"]
