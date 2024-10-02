use cli_clipboard::{ClipboardContext, ClipboardProvider};
use reqwest::header::HeaderMap;
use tokio::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;
use clap::Parser;
use std::time::SystemTime;
const TARGET_PHRASE: &str = "27b47455f301788ebf9f85d0d1aa90d5";

use reqwest::Client;
use serde_json::json;

use crate::utils::Config;

const API_URL: &str = "https://labs.hackthebox.com/";


#[tokio::main]
pub async fn start_daemon() {
    let now = SystemTime::now();


    
    let mut ctx = ClipboardContext::new().unwrap();
    let mut last_content = String::new();
    match now.elapsed() {
        Ok(time) => println!("Time: {}", time.as_secs() / 60 / 60),
        Err(e) => eprintln!("SHit {}", e),
    }
    println!("\nDaemon started. Listening for flags\n");

    loop {
        thread::sleep(Duration::from_millis(100));

        let content = match ctx.get_contents() {
            Ok(content) => content,
            Err(_) => continue,
        };
        if content.contains(" ") {
            continue;
        }

        if content.len() == TARGET_PHRASE.len() && content != last_content {
            last_content = content.clone();
            if is_valid_hex(&content) {
                println!("Sending Flag");
                let result = submit_flag(&content).await;
                if result {
                    println!("Congrats the flag was submited!!");
                    match now.elapsed() {
                        Ok(time) => println!("Time: {}", time.as_secs() / 60 / 60),
                        Err(e) => eprintln!("SHit {}", e),
                    }
                } else {
                    println!("Sorry Wrong Flag :( or something happened, try submitting the flag and reporting to me about what happened.")
                }
            } 
        }
    }
}

fn is_valid_hex(s: &str) -> bool {
    for c in s.chars() {
        match c {
            '0'..='9' | 'a'..='f' | 'A'..='F' => continue,
            _ => return false,
        }
    }
    true
}


async fn submit_flag(flag: &str) -> bool {
    let client = Client::new();
    let url = format!("{}api/v4/arena/own", API_URL);
    let submit_flag_request = json!({
        "flag": flag,
    });

    let toml_string = fs::read_to_string("config/default.toml").await.expect("No Config file found");
    let config: Config = toml::from_str(toml_string.as_str()).unwrap();

    let token = config.htb["api_token"].clone();

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());
    headers.insert("Content-Type", "application/json".to_string().parse().unwrap());

    let response = client
        .post(&url)
        .headers(headers)
        .json(&submit_flag_request)
        .send()
        .await;

    match response {
        Ok(res) => {
            if let Ok(text) = res.text().await {
                println!("{}", text);
                text.contains("is now owned.")
            } else {
                false
            }
        }
        Err(_) => false,
    }
}
