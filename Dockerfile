FROM rust:1-buster as builder

WORKDIR /usr/src/echo

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release 
RUN rm -rf target

# Build app
COPY src ./src
RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/src/echo
COPY --from=builder /usr/src/echo/target/release/echo .
CMD ["./echo"]
