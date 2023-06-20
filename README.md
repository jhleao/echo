# Echo ➰

A small utility to echo and print incoming HTTP requests.

<p align="center">
<img src="./.github/demo.gif">
</p>

## Usage

```bash
# Replace 3000 with the port you want to listen on
docker run --rm -it -p 3000:1 jhleao/echo
```

Any incoming HTTP request will be responded with status 200, plus the same headers and body as the request.

> As per [the spec](https://www.rfc-editor.org/rfc/rfc7231#section-7.1.1.2), the server additionally returns a `date` header with the time of response.

> This tool is meant for debugging usage and should NOT be used in production environments. Unlike traditional echo servers, this loads incoming data into memory instead of piping the stream through, in order to perform the formatting and printing.

## Development

Make sure to install the Rust toolchain (e.g. with [rustup](https://rustup.rs/)) and run the following to start the server in development mode:

```bash
PORT=3000 cargo run
```

Building and releasing is done automatically within GitHub Actions.
