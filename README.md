# Echo

A small utility to echo and inspect incoming HTTP requests.

## Usage

```bash
# Replace 3000 with the port you want to listen on
docker run --rm -p 3000:1 jhleao/echo
```

This is meant for debugging usage and should NOT be used in production environments.

Unlike traditional echo servers, this loads incoming data into memory instead of piping the stream through. This is to power the formatting and logging features.

## Development

Make sure to install the Rust toolchain (e.g. with [rustup](https://rustup.rs/)) and run the following to start the server in development mode:

```bash
PORT=3000 cargo run
```

Building and releasing is done automatically with GitHub Actions.
