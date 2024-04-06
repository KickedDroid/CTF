use std::{fs, io::{BufRead, BufReader}, net::TcpStream, os::unix::thread, process::Command, thread::Thread};
use rayon::{prelude::*, spawn};


pub(crate) async fn subdomain_enum(url: String) {
    let subdomain_enum_cmd = format!("./subscout -t {}", url.clone());
    let subd_enum_result = Command::new("./subscoutb")
        .args([
            "-t",
            url.clone().as_str(),
        ])
        .output()
        .expect("Subdomain Enumeration failed...");
    println!("Subdomain Enumeration Results ---------------------------------");

    print!("{}", String::from_utf8_lossy(&subd_enum_result.stdout));
    println!("End of Subdomain Enumeration Results --------------------------\n")        
}

pub(crate) async fn subdomain_enum_parallel(url: String, wordlist: String) {
    let subdomains = read_subdomains_from_file(wordlist.as_str());
    subdomains.par_iter().for_each(|subdomain| {
        let full_domain = format!("{}.{}", subdomain, url.clone());
       //print!("Checking: {}\n", full_domain);
        
            spawn(move || {
                match reqwest::blocking::get(format!("http://{}", full_domain).as_str()) {
                    Ok(res) => println!("{}\n{}", full_domain, res.status()),
                    Err(_) => (),
                }
            });

    });
}


fn read_subdomains_from_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Failed to read file");
    let mut subdomains = Vec::new();

    for line in BufReader::new(contents.as_bytes()).lines() {
        if let Ok(subdomain) = line {
            subdomains.push(subdomain);
        }
    }

    subdomains
}