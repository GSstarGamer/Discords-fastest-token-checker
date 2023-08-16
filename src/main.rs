extern crate reqwest;
extern crate tokio;

use reqwest::header;
use std::fs;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

async fn check_token(
    client: reqwest::Client,
    token: String,
    sender: mpsc::Sender<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("{}", token))?,
    );

    let response = client
        .get("https://discord.com/api/v10/users/@me")
        .headers(headers)
        .send()
        .await?;

    if response.status().is_success() {
        sender.send(token).await.unwrap();
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tokens = fs::read_to_string("tokens.txt")?;
    let tokens: Vec<String> = tokens.lines().map(|s| s.to_string()).collect();

    let client = reqwest::Client::new();
    let (sender, mut receiver) = mpsc::channel::<String>(tokens.len());

    for token in tokens {
        let sender_clone = sender.clone();
        let client_clone = client.clone();

        tokio::spawn(async move {
            if let Err(err) = check_token(client_clone, token.clone(), sender_clone).await {
                eprintln!("Error checking token: {}", err);
            }
        });
    }

    drop(sender); // Drop the sender to indicate no more tokens will be sent

    let valid_tokens: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    while let Some(valid_token) = receiver.recv().await {
        valid_tokens.lock().unwrap().push(valid_token);
    }

    println!("Valid tokens: {:?}", valid_tokens.lock().unwrap());

    Ok(())
}
