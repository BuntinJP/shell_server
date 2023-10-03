use rand::seq::SliceRandom;
use rand::Rng;

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

/* fn main() {
    let token = generate_token(64); // 64文字のトークンを生成
    println!("Generated token: {}", token);
} */
