mod utils;
mod nmap;
mod subdenum;
use std::env;
use std::process::{Command, Output, Stdio};
#[tokio::main]
async fn main() ->  Result<(), std::io::Error> {
    utils::display_name();
    let args: Vec<String> = env::args().collect();
    let ip = &args[1];

    println!("Running scans on {:?} using nmap and subscout....", ip);

    nmap::nmap_scan(ip.clone());
    subdenum::subdomain_enum(ip.clone());

    
    Ok(())
}


