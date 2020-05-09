use std::io::prelude::*;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Instant::now();

    task("task1", now.clone())?;
    task("task2", now.clone())?;
    task("task3", now.clone())?;
    Ok(())
}

fn task(label: &str, now: std::time::Instant) -> Result<(), Box<dyn std::error::Error>> {
    // Simulate network delay using thread sleep for 2 seconds
    println!(
        "OS Thread {:?} - {} started: {:?}",
        std::thread::current().id(),
        label,
        now.elapsed(),
    );
    sleep(std::time::Duration::from_secs(2));

    // Write to server - server will echo this back to us with 8 second delay
    let mut stream = TcpStream::connect("127.0.0.1:6142")?;
    stream.write_all(label.as_bytes())?;
    println!(
        "OS Thread {:?} - {} written: {:?}",
        std::thread::current().id(),
        label,
        now.elapsed()
    );

    // Read 5 chars we expect (to avoid dealing with EOF, etc.)
    let mut buffer = [0; 5];
    stream.read_exact(&mut buffer)?;
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
