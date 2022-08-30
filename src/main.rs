#![allow(unused_imports)]
use salvo::prelude::*;
use serde::{Serialize, Deserialize};
use reqwest;




#[handler]
async fn hello_world() -> &'static str {
    "Hello world"
}


#[tokio::main]
async fn main() {
    let router = Router::new().get(hello_world);
    println!("listening at 127.0.0.1:7878");
    Server::new(TcpListener::bind("127.0.0.1:7878")).serve(router).await;
}