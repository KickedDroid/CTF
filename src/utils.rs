use std::collections::HashMap;
use clap::{Parser};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub wordlist: HashMap<String, String>,
    pub zap: HashMap<String, String>,
    pub htb: HashMap<String, String>
}


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    /// HTB Token
    #[clap(short, long)]
    token: String,
}