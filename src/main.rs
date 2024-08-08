mod utils;
mod nmap;
mod subdenum;
mod fuzzing;
mod whowhat;
use std::{env, os::unix::thread};
use clap::{arg};
use std::process::Command;


fn main() ->  Result<(), std::io::Error> {
    //utils::display_name();
    let args: Vec<String> = env::args().collect();
    let mut ip = &args[1];
    println!("Performing recon on {:?}\n", ip);

    let rustscan = Command::new("rustscan").args([
        "-a",
        ip.clone().as_str(),
        "--",
        "-sC",
        "-sV"
    ]).output()
    .expect("Rustscan failed...");

    print!("{}", String::from_utf8_lossy(&rustscan.stdout));

    let ferox = fuzzing::fuzz(ip.to_string());
    let ffuf = Command::new("ffuf");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::utils;

    #[test]
    fn test_nmap() {
        let ip = "ipfs.io";
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            super::nmap::nmap_scan(ip.to_string()).await;
        });
    }
    #[test]
    fn test_subdomain_enum() {
        let url = "google.com";
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            super::subdenum::subdomain_enum(url.to_string()).await;
        });
    }
    #[test]
    fn test_fuzz() {
        let url = "google.com";
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            super::fuzzing::fuzz(url.to_string()).await;
        });
    }
    #[test]
    fn test_whowhat() {
        let pb = utils::display_progressbar();
        let url = "google.com";
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            super::whowhat::whowhat(url.to_string()).await;
        });
        pb.finish()
    }
    #[test]
    fn test_subdomain_enum_parallel() {
        let url = "kaleido.ai";
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            super::subdenum::subdomain_enum_parallel(url.to_string(), "wordlists/subdomains-top1million-5000.txt".to_owned()).await;
        });
    }
}
