use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use mini_redis::Command::{self, Get, Set};
use bytes::Bytes;


type Db = Arc<Mutex<HashMap<String, Bytes>>>;


#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening"); 

    let db = Arc::new(Mutex::new(HashMap::new())); 

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // Clone the handle to the hash map.
        let db = db.clone();

        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {

    let mut connection = Connection::new(socket); 


    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap(){
            Set(cmd) => {
                let mut db = db.lock().unwrap(); 

                db.insert(cmd.key().to_string(), cmd.value().clone()); 
                Frame::Simple("OK".to_string())
            }

            Get(cmd) => { 
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }

            cmd => panic!("unimplemented {:?}", cmd),

        };
        connection.write_frame(&response).await.unwrap();


    }
    




}
