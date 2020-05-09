# async-rust-example

Example of async programming in Rust, written for [this blog post](http://jamesmcm.github.io/blog/2020/05/06/a-practical-introduction-to-async-programming-in-rust/#en).

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
