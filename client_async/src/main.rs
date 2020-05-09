#[allow(unused_imports)]
use futures::stream::futures_unordered::FuturesUnordered;
#[allow(unused_imports)]
use futures::stream::StreamExt;
use std::error::Error;
use std::thread::sleep;
use std::time::Instant;
#[allow(unused_imports)]
use tokio::join;
use tokio::net::TcpStream;
use tokio::prelude::*;

// Synchronous: (2+8+4) * 3 = 42 secs
// Async single-thread: 2 + 8 + (3*4) = 22 secs
// Async multi-thread (>3 threads): 2 + 8 + 4 = 14 secs

// #[tokio::main(core_threads = 1, max_threads = 1)] // Single thread
//
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let now = Instant::now();

    // Synchronous
    // task("task1", now.clone()).await?;
    // task("task2", now.clone()).await?;
    // task("task3", now.clone()).await?;

    // Asynchronous single-thread
    // let mut futs = FuturesUnordered::new();

    // futs.push(task("task1", now.clone()));
    // futs.push(task("task2", now.clone()));
    // futs.push(task("task3", now.clone()));

    // while let Some(_handled) = futs.next().await {}

    // Asynchronous multi-threaded
    // let mut futs = FuturesUnordered::new();
    // futs.push(tokio::spawn(task("task1", now.clone())));
    // futs.push(tokio::spawn(task("task2", now.clone())));
    // futs.push(tokio::spawn(task("task3", now.clone())));
    // while let Some(_handled) = futs.next().await {}

    // Equivalent to FuturesUnordered, but without allocation, less wieldy for many futures
    match join!(
        tokio::spawn(task("task1", now.clone())),
        tokio::spawn(task("task2", now.clone())),
        tokio::spawn(task("task3", now.clone()))
    ) {
        (x, y, z) => {
            // dbg!("{:?}", (&x, &y, &z));
            (x.ok(), y.ok(), z.ok())
        }
    };
    Ok(())
}

async fn task(
    label: &'static str,
    now: std::time::Instant,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Simulate network delay using Tokio async delay for 2 seconds
    println!(
        "OS Thread {:?} - {} started: {:?}",
        std::thread::current().id(),
        label,
        now.elapsed(),
    );
    tokio::time::delay_for(tokio::time::Duration::from_secs(2)).await;

    // Write to server - server will echo this back to us with 8 second delay
    let mut stream = TcpStream::connect("127.0.0.1:6142").await?;
    stream.write_all(label.as_bytes()).await?;
    println!(
        "OS Thread {:?} - {} written: {:?}",
        std::thread::current().id(),
        label,
        now.elapsed()
    );

    // Read 5 chars we expect (to avoid dealing with EOF, etc.)
    let mut buffer = [0; 5];
    stream.read_exact(&mut buffer).await?;
    stream.shutdown(std::net::Shutdown::Both)?;
    println!(
        "OS Thread {:?} - {} read: {:?}",
        std::thread::current().id(),
        label,
        now.elapsed()
    );

    // Simulate computation work by sleeping actual thread for 4 seconds
    sleep(std::time::Duration::from_secs(4));
    println!(
        "OS Thread {:?} - {} finished: {:?}",
        std::thread::current().id(),
        std::str::from_utf8(&buffer)?,
        now.elapsed()
    );
    Ok(())
}
