FROM rust:1.56.1-alpine AS build

RUN apk add --no-cache \
        openssl-dev musl-dev glib-dev cairo-dev pkgconfig gdk-pixbuf-dev webkit2gtk-dev curl \
        libappindicator-dev patchelf librsvg-dev gtk+3.0-dev nodejs npm make
ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN node --version && \
    npm --version && \
    # install yarn
    npm install --global yarn && \
    yarn --version && \
    cargo install tauri-cli --version ^1.0.0-beta

WORKDIR /src
COPY . .
WORKDIR /src/frontend
RUN yarn
RUN cargo tauri build
