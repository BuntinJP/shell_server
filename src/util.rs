use std::vec::Vec;

use rand::seq::SliceRandom;
use rand::Rng;
use reqwest::Error;
use serde::Deserialize;

pub fn generate_token(length: usize) -> String {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                           abcdefghijklmnopqrstuvwxyz\
                           0123456789\
                           !\"#$%&'()*+-./:;<=>?@[\\]^_`{|}~"; // カンマを除外

    let token: String = (0..length)
        .map(|_| *charset.choose(&mut rand::thread_rng()).unwrap() as char)
        .collect();
    token
}
#[derive(Deserialize)]
struct Response {
    success: bool,
    result: Result,
}

#[derive(Deserialize)]
struct Result {
    query: String,
    results: Vec<KeyValue>,
}

#[derive(Deserialize)]
struct KeyValue {
    KeyName: String,
    KeyValue: String,
}

/* fn main() {
    let token = generate_token(64); // 64文字のトークンを生成
    println!("Generated token: {}", token);
} */

/*
https://dataworker.buntin.workers.dev/db/keys/all
*/

pub async fn get_passwords() -> std::result::Result<Vec<String>, Error> {
    let url = "https://dataworker.buntin.workers.dev/db/keys/all";

    let client = reqwest::Client::new();
    match client.get(url).send().await {
        Ok(resp) => match resp.json::<Response>().await {
            Ok(response) => {
                if response.success {
                    let mut passwords = Vec::new();
                    for kv in response.result.results {
                        println!("KeyName: {}, KeyValue: {}", kv.KeyName, kv.KeyValue);
                        passwords.push(kv.KeyValue);
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
