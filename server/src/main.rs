use futures_util::io::AsyncReadExt;
use futures::stream::StreamExt;
use async_std::net::TcpListener;
use std::time::Duration;

#[async_std::main]
async fn main() {
    let addr = "127.0.0.1:6142";
    let listener = TcpListener::bind(addr).await.unwrap();

    let server = {
        async move {
            let mut incoming = listener.incoming();
            while let Some(conn) = incoming.next().await {
                match conn {
                    Err(e) => eprintln!("accept failed = {:?}", e),
                    Ok(sock) => {
                        async_std::task::spawn(async move {
                            let (mut reader, mut writer) = sock.split();
                            async_std::task::sleep(Duration::from_secs(8)).await;
                            match async_std::io::copy(&mut reader, &mut writer).await {
                                Ok(amt) => {
                                    println!("wrote {} bytes", amt);
                                }
                                Err(err) => {
                                    eprintln!("IO error {:?}", err);
                                }
                            }
                        });
                    }
                }
            }
        }
    };
    println!("Server running on localhost:6142");
    server.await;
}
