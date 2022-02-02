# buildah bud --layers -t udptest:1.0 .
# podman build (--layers|--no-cache) -t udptest:1.0 .

FROM rust:1.58-alpine as builder
RUN apk add --no-cache --quiet musl-dev
WORKDIR /stuff
COPY Cargo.toml Cargo.lock ./
COPY src src/
COPY examples examples/
RUN cargo install --quiet --root /usr/ --example udptest --path .

FROM alpine
COPY --from=builder /usr/bin/udptest /usr/bin
ENTRYPOINT ["/usr/bin/udptest", "--help"]
