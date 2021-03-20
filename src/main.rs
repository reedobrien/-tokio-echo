// use tokio::fs::File;
// use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    // let _ = read_some().await?;
    // let _ = read_all().await?;
    // let _ = write_some().await?;
    // let _ = write_all().await?;
    // let _ = copy().await?;

    listen_copy().await?;

    Ok(())
}

async fn listen_copy() -> io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();

    loop {
        let (mut sock, _) = listener.accept().await?;
        tokio::spawn(async move {
            let (mut rd, mut wr) = sock.split();
            io::copy(&mut rd, &mut wr).await.unwrap()
        });
    }
}

async fn _listen_manual_copy() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        if socket.write_all(&buf[..n]).await.is_err() {
                            return;
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
            }
        });
    }
}

// Warmup functions for I/O tutorial in tokio tutorial.
// async fn read_some() -> io::Result<()> {
//     let mut f = File::open("/Users/robrien/tmp/foo.txt").await?;
//     let mut buffer = [0; 10];

//     let n = f.read(&mut buffer[..]).await?;

//     println!("The bytes: {:?}", &buffer[..n]);
//     Ok(())
// }

// async fn read_all() -> io::Result<()> {
//     let mut f = File::open("/Users/robrien/tmp/foo.txt").await?;
//     let mut buffer = Vec::new();

//     let _ = f.read_to_end(&mut buffer).await?;

//     println!("The bytes: {:?}", &buffer);
//     Ok(())
// }

// async fn write_some() -> io::Result<()> {
//     let mut file = File::create("/Users/robrien/tmp/bar.txt").await?;

//     let n = file
//         // Needs to be longer to actuallt not finish.
//         .write(b"some bytes, and some more and more and more")
//         .await?;

//     println!("Wrote the first {} bytes", n);
//     Ok(())
// }

// async fn write_all() -> io::Result<()> {
//     let mut buffer = File::create("/Users/robrien/tmp/bar.txt").await?;

//     buffer
//         .write_all(b"some more bytes, that will all get written for sure")
//         .await?;

//     Ok(())
// }

// async fn copy() -> io::Result<()> {
//     let mut reader: &[u8] = b"hello";
//     let mut file = File::create("foo.txt").await?;

//     io::copy(&mut reader, &mut file).await?;

//     Ok(())
// }
