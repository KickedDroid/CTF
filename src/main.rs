mod utils;
mod nmap;
mod subdenum;
mod fuzzing;
use std::{env, os::unix::thread};
use tokio::select;

#[tokio::main]
async fn main() ->  Result<(), std::io::Error> {
    utils::display_name();
    let args: Vec<String> = env::args().collect();
    let ip = &args[1];

    println!("Performing recon on {:?}", ip);

    let t1 =tokio::spawn(nmap::nmap_scan(ip.clone()));
    let t2 = tokio::spawn(subdenum::subdomain_enum(ip.clone()));
    let t3 = tokio::spawn(fuzzing::fuzz(ip.clone()));
    
    t1.await.unwrap();
    t2.await.unwrap();
    t3.await.unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
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
}