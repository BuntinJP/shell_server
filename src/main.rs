mod endpoints;
mod middleware;
mod types;
mod util;

use actix_rt::main;
use actix_web::{web::Data, App, HttpServer};
use endpoints::{HelloWorld, KeysRegister, Sudo, Users};
use env_logger::Builder;
use log::{info, LevelFilter};
use rand::Rng;
use std::env;
use std::sync::Arc;

/*
https://dataworker.buntin.workers.dev/db/keys/all
*/

#[main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    Builder::new().filter(None, LevelFilter::Info).init();
    let master_password = match env::var("SS_MASTER_PASSWORD") {
        Ok(value) => {
            info!("MASTER_PASSWORD set to {}", value);
            value
        }
        Err(_) => {
            let random_password: String = rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(10)
                .map(char::from)
                .collect();
            info!("Generated random MASTER_PASSWORD: {}", random_password);
            random_password
        }
    };

    let pss = util::get_passwords().await.unwrap();
    let password_cache = Arc::new(pss);

    env::set_var("MASTER_PASSWORD", &master_password); // Set the master password

    let port = env::var("SS_PORT").unwrap_or_else(|_| String::from("54321"));
    info!("Port set to {}", port);
    let bind_address = format!("127.0.0.1:{}", port);
    info!("Binding to {}", bind_address);

    //print server settings
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(password_cache.clone()))
            .configure(HelloWorld::configure)
            .configure(KeysRegister::configure)
            .configure(Users::configure)
            .configure(Sudo::configure)
    })
    .bind(bind_address)?
    .run()
    .await
}
