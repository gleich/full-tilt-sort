# hadolint ignore=DL3007
FROM rust:latest AS builder

# Meta data
LABEL maintainer="email@mattglei.ch"
LABEL description="🚀 Really fast file sorting CLI"

# Copying over all the files
COPY . /usr/src/app
WORKDIR /usr/src/app

# Build binary
RUN cargo install --force cargo-make
RUN cargo make build-rust-dev

# hadolint ignore=DL3006,DL3007
FROM alpine:latest
WORKDIR /
COPY --from=builder /usr/src/app/target/debug/full-tilt-sort .
CMD ["./full-tilt-sort"]
