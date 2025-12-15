use std::env;
use std::net::SocketAddr;

use log::debug;
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

async fn process(socket: TcpStream, userdata: SocketAddr) {
    // Make connection
    let mut conn: Connection = Connection::new(socket);

    while let Some(frame) = conn.read_frame().await.unwrap() {
        println!("Current frame: {:?}", frame);

        let res = Frame::Simple(userdata.to_string());
        conn.write_frame(&res).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    // Process command arguments
    let env_args: Vec<String> = env::args().collect();
    debug!("Server args: {:?}", env_args);

    let uri_default: &String = &String::from("127.0.0.1:6700");
    let uri: &String = env_args
        .get(1)
        .unwrap_or(uri_default);

    // Set up server
    let listener = TcpListener::bind(uri).await.unwrap();

    loop {
        let (socket, userdata) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket, userdata).await;
        });
    }
}
