use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let _ = read_some().await;
    let _ = read_all().await;
    let _ = write_some().await;
    let _ = write_all().await;
    Ok(())
}

async fn read_some() -> io::Result<()> {
    let mut f = File::open("/Users/robrien/tmp/foo.txt").await?;
    let mut buffer = [0; 10];

    let n = f.read(&mut buffer[..]).await?;

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}

async fn read_all() -> io::Result<()> {
    let mut f = File::open("/Users/robrien/tmp/foo.txt").await?;
    let mut buffer = Vec::new();

    let _ = f.read_to_end(&mut buffer).await?;

    println!("The bytes: {:?}", &buffer);
    Ok(())
}

async fn write_some() -> io::Result<()> {
    let mut file = File::create("/Users/robrien/tmp/bar.txt").await?;

    let n = file
        // Needs to be longer to actuallt not finish.
        .write(b"some bytes, and some more and more and more")
        .await?;

    println!("Wrote the first {} bytes", n);
    Ok(())
}

async fn write_all() -> io::Result<()> {
    let mut buffer = File::create("/Users/robrien/tmp/bar.txt").await?;

    buffer
        .write_all(b"some more bytes, that will all get written for sure")
        .await?;
    Ok(())
}
