mod utils;
mod nmap;
mod subdenum;
mod fuzzing;
mod whowhat;
mod zap;
use std::{env, os::unix::thread};
use clap::{arg};
use zap::ZapScanner;
use std::process::Command;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <IP> <DOMAIN>", args[0]);
        std::process::exit(1);
    }

    let ip = &args[1];
    let domain = &args[2];

    println!("IP set to: {}", ip);
    println!("DOMAIN set to: {}", domain);

    //whowhat(domain.clone()).await;
    // Rustscan
    println!("\nRunning Rustscan...");
    let rustscan_output = Command::new("rustscan")
        .args(&["-a", ip,"-g","--", "-sV", "-sC"])
        .output()?;
    println!("{}", String::from_utf8_lossy(&rustscan_output.stdout));

    // Nuclei
    println!("\nRunning Nuclei...");
    let nuclei_output = Command::new("nuclei")
        .args(&["-i", ip])
        .output()?;
    println!("{}", String::from_utf8_lossy(&nuclei_output.stdout));

    // ffuf
    println!("\nRunning ffuf...");
    let ffuf_output = Command::new("ffuf")
        .args(&[
            "-w", "/home/nacho/Documents/tools/SecLists/Discovery/DNS/subdomains-top1million-5000.txt:FUZZ",
            "-u", &format!("http://{}", domain),
            "-H", &format!("Host: FUZZ.{}", domain),
        ])
        .output()?;
    println!("{}", String::from_utf8_lossy(&ffuf_output.stdout));


    let zap_api_url = "http://localhost:8080";
    let api_key = env::var("ZAP_API_KEY").expect("ZAP_API_KEY not set in environment");

    let scanner = ZapScanner::new(zap_api_url.to_string(), api_key);
    let target_url = format!("http://{}",domain.clone());
    let scan_results = scanner.scan_target(&target_url).await?;

    println!("Scan Alerts:\n{}", scan_results);

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
