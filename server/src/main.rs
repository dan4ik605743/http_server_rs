use anyhow::{Context, Result};
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::mpsc,
    thread,
    time::Duration,
};
use threadpool::ThreadPool;
fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let pool = ThreadPool::build(4)?;
    let (tx, rx): (mpsc::Sender<Result<()>>, mpsc::Receiver<Result<()>>) = mpsc::channel();

    thread::spawn(move || {
        rx.iter().for_each(|data| {
            if let Err(e) = data {
                eprintln!("Error: {e}");
            }
        });
    });

    listener.incoming().try_for_each(|stream| -> Result<()> {
        let stream = stream?;

        let tx = tx.clone();
        pool.execute(move || {
            let _ = tx.send(handle_connection(stream));
        });

        Ok(())
    })?;

    println!("Shutting down");
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let buff = BufReader::new(&mut stream);

    let (status_line, filename) = match buff
        .lines()
        .next()
        .context("Failed to read line user")??
        .as_str()
    {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 400 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename)?;
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    Ok(stream.write_all(response.as_bytes())?)
}
