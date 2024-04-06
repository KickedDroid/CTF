mod utils;
mod nmap;
mod subdenum;
mod fuzzing;
mod whowhat;
use std::{env, os::unix::thread};
use tokio::select;

#[tokio::main]
async fn main() ->  Result<(), std::io::Error> {
    utils::display_name();
    let args: Vec<String> = env::args().collect();
    let ip = &args[1];

    println!("Performing recon on {:?}\n", ip);

    let t1 =tokio::spawn(nmap::nmap_scan(ip.clone()));
    let t2 = tokio::spawn(subdenum::subdomain_enum(ip.clone()));
    let t3 = tokio::spawn(fuzzing::fuzz(ip.clone()));
    let t4 = tokio::spawn(whowhat::whowhat(ip.clone()));
    
    t1.await.unwrap();
    t2.await.unwrap();
    t3.await.unwrap();
    t4.await.unwrap();
    

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
        let url = "google.com";
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            super::subdenum::subdomain_enum_parallel(url.to_string(), "wordlists/subdomains-top1million-5000.txt".to_owned()).await;
        });
    }
}
