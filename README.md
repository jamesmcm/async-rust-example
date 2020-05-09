# async-rust-example

Example of async programming in Rust, written for [this blog post](TODO).

## Server

Run the server with:

```bash
$ cargo run --release --bin server
```

## Clients

Then run the different clients with:

```bash
$ cargo run --release --bin client_synchronous
$ cargo run --release --bin client_async
$ cargo run --release --bin client_synchronous_parallel
```

While the echo server is running.
