use std::vec::Vec;
extern crate dotenv;
use dotenv::dotenv;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct Response {
    success: bool,
    result: Result,
}

#[derive(Deserialize)]
struct Result {
    _query: String,
    results: Vec<KeyValue>,
}

#[derive(Deserialize)]
struct KeyValue {
    key_name: String,
    key_value: String,
}

/* fn main() {
    let token = generate_token(64); // 64文字のトークンを生成
    println!("Generated token: {}", token);
} */

/*
https://dataworker.buntin.workers.dev/db/keys/all
*/

pub async fn get_passwords() -> std::result::Result<Vec<String>, Error> {
    dotenv().ok();
    let url = "https://dataworker.buntin.workers.dev/db/keys/all";

    let client = reqwest::Client::new();
    let value = match dotenv::var("WORKER_PASSWORD") {
        Ok(value) => {
            value
        }
        Err(_) => {
            //stop the program
            panic!("WORKER_PASSWORD is not set");
        }
    };
    match client.get(url).header(reqwest::header::AUTHORIZATION, value).send().await {
        Ok(resp) => match resp.json::<Response>().await {
            Ok(response) => {
                if response.success {
                    let mut passwords = Vec::new();
                    for kv in response.result.results {
                        println!("a key setted(name:{},value:{})", kv.key_name, kv.key_value);
                        passwords.push(kv.key_value);
                    }
                    Ok(passwords)
                } else {
                    eprintln!("Failed to retrieve data");
                    Ok(vec![]) // Success is false, returning empty vector
                }
            }
            Err(e) => {
                eprintln!("Failed to parse JSON: {:?}", e);
                Err(e)
            }
        },
        Err(e) => {
            eprintln!("Failed to send request: {:?}", e);
            Err(e)
        }
    }
}
