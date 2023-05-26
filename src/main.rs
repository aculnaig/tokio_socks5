use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Listening from 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 3];

            loop {
                socket.read(&mut buf).await;

                /* Currently supported method authentication:
                 *   - X'00' NO AUTHENTICATION REQUIRED
                 */
                match buf {
                    [0x05, _, _] => {
                        socket.write_all(&[0x05, 0x00]).await;
                        ();
                    }
                    _ => {
                        socket.write_all(&[0x05, 0xff]).await;
                        ();
                    }
                }

                let mut buf = [0; 6];
                socket.read(&mut buf).await;

                match buf {
                    [0x05, cmd, 0x00, atyp, dst_addr, dst_port] => {
                        // Send a X'01' general SOCKS server failure for now
                        socket.write_all(&[0x05, 0x01, 0x00, atyp, dst_addr, dst_port]).await;
                        ();
                    }
                    _ => {
                        socket.write_all(&[0x05, 0x0ff]).await;
                        ();
                    }
                }
            }
        });
    }
}
