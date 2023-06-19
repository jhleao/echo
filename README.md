# Echo

A small utility to echo and inspect incoming HTTP requests.

## Usage

```bash
# Replace 3000 with the port you want to listen on
docker run --rm -p 3000:1 jhleao/echo
```

This is meant for debugging usage and should NOT be used in production environments.

## Development

Make sure to install the Rust toolchain (e.g. with [rustup](https://rustup.rs/)) and run the following to start the server in development mode:

```bash
cargo run
```

Building and releasing is done automatically with GitHub Actions.
