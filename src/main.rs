mod utils;
mod nmap;
mod subdenum;
mod fuzzing;
use std::env;
use std::process::{Command, Output, Stdio};
#[tokio::main]
async fn main() ->  Result<(), std::io::Error> {
    utils::display_name();
    let args: Vec<String> = env::args().collect();
    let ip = &args[1];

    //println!("Running scans on {:?} using nmap and subscout....", ip);

    nmap::nmap_scan(ip.clone());
    subdenum::subdomain_enum(ip.clone());
    fuzzing::fuzz(ip.clone());
    
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_nmap() {
        let ip = "127.0.0.1";
        super::nmap::nmap_scan(ip.to_string());
    }
    #[test]
    fn test_subdomain_enum() {
        let url = "google.com";
        super::subdenum::subdomain_enum(url.to_string());
    }
    #[test]
    fn test_fuzz() {
        let url = "google.com";
        super::fuzzing::fuzz(url.to_string());
    }
}