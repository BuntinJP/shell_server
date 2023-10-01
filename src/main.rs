mod endpoints;

use actix_rt::main;
use actix_web::{App, HttpServer};
use endpoints::{HelloWorld, KeysRegister};
use std::env;

#[main]
async fn main() -> std::io::Result<()> {
    env::set_var("MASTER_PASSWORD", "test1234"); // Set the master password

    let port = env::var("PORT").unwrap_or_else(|_| String::from("54321"));
    let bind_address = format!("127.0.0.1:{}", port);

    HttpServer::new(|| {
        App::new()
            .configure(HelloWorld::configure)
            .configure(KeysRegister::configure)
    })
    .bind(bind_address)?
    .run()
    .await
}
