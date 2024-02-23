use core::panic;
use mini_redis::{Command, Connection, Frame};
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (stream, connection) = listener.accept().await.unwrap();
        println!("{:?}", connection);
        tokio::spawn(async move {
            process_stream(stream).await;
        });
    }
}

async fn process_stream(socket: TcpStream) {
    let mut connection = Connection::new(socket);
    let mut db = HashMap::new();

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response: Frame = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("Ok".to_string())
            }

            Command::Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Error("Not founded".to_string())
                }
            }

            cmd => {
                panic!("Unimplemented {:?}", cmd)
            }
        };

        println!("{:#?}", db);
        connection.write_frame(&response).await.unwrap();
    }
}
