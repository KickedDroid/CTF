mod fuzzing;
mod kronos;
mod nmap;
mod utils;
mod zap;
use serde::de::value::Error;
use std::env;
use std::process::Command;
use std::thread;
use tokio::fs;
use utils::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <IP> <DOMAIN>", args[0]);
        std::process::exit(1);
    }

    let mut ip = &args[1];
    let ip = ip.to_string().clone();
    let mut domain = &args[2];
    let domain = domain.to_string().clone();

    println!("IP set to: {}", ip);
    println!("DOMAIN set to: {}", domain);

    //whowhat(domain.clone()).await;
    // Rustscan
    println!("\nRunning Rustscan...");
    let rustscan_output = Command::new("rustscan")
        .args(&["-a", &ip, "-g", "--ulimit", "5000"])
        .output();
    match rustscan_output {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            process_ports(
                String::from_utf8_lossy(&output.stdout).to_string(),
                ip.clone(),
            );
        },
        Err(e) => eprintln!("Rustscan Failed {e}")
    }
    thread::spawn(move || {
        kronos::start_daemon();
    });

    loop {}
    Ok(())
}

fn process_ports(results: String, ip: String) {
    let ip = ip.clone();
    let is_smb = results.contains("445") | results.contains("139");
    let is_ftp = results.contains("20") | results.contains("21");
    let is_http = results.contains("80") | results.contains("8080");
    let host = ip.clone();
    match is_smb {
        true => {
            thread::spawn(move || {
                println!("Found SMB Server. Running netexec...");
                // Nuclei
                let netexec_rid = Command::new("netexec")
                    .args(&[
                        "smb",
                        ip.clone().as_str(),
                        "-u",
                        "\"guest\"",
                        "-p",
                        "",
                        "--rid-brute",
                        "--threads",
                        "5",
                    ])
                    .output();
                match netexec_rid {
                    Ok(output) => {
                        println!(
                            "\n**SMB**\n**RIDS**\n{}\n***RIDS***",
                            String::from_utf8_lossy(&output.stdout)
                        );
                    }
                    Err(e) => println!("netexec RIDS SHIDDED {}", e),
                }
                let netexec_shares = Command::new("netexec")
                    .args(&[
                        "smb",
                        ip.clone().as_str(),
                        "-u",
                        "\"guest\"",
                        "-p",
                        "",
                        "--shares",
                        "--threads",
                        "5",
                    ])
                    .output();
                match netexec_shares {
                    Ok(output) => {
                        println!(
                            "\n**SHARES**\n{}\n***SHARES***",
                            String::from_utf8_lossy(&output.stdout)
                        );
                    }
                    Err(e) => println!("netexec SHARES SHIDDED {}", e),
                }
                let netexec_dump = Command::new("netexec")
                    .args(&[
                        "smb",
                        ip.clone().as_str(),
                        "-u",
                        "\"guest\"",
                        "-p",
                        "",
                        "--threads",
                        "5",
                        "-M",
                        "spider_plus",
                        "-o",
                        "DOWNLOAD_FLAGS=True",
                        "OUTPUT_FOLDER=.",
                    ])
                    .output();
                match netexec_dump {
                    Ok(output) => {
                        println!(
                            "\n**DUMP**\n{}\n***DUMP***\n\n***SMB***",
                            String::from_utf8_lossy(&output.stdout)
                        );
                        let grep = Command::new("netexec")
                            .args(&[
                                "grep",
                                "-r",
                                "password",
                                "."
                            ])
                            .output();
                        match grep {
                            Ok(output) => {
                                println!(
                                    "\n{}\n",
                                    String::from_utf8_lossy(&output.stdout)
                                );
                            }
                            Err(e) => println!("netexec DUMP SHIDDED {}", e),
                        }
                    }
                    Err(e) => println!("netexec DUMP SHIDDED {}", e),
                }
            });
        }
        false => {}
    }

    match is_http {
        true => {
            thread::spawn(move || {
                // Nuclei
                let nuclei_output = Command::new("nuclei").args(&["-u", &host.clone()]).output();
                match nuclei_output {
                    Ok(output) => {
                        println!(
                            "\n**NUCLEI**\n{}\n***NUCLEI***",
                            String::from_utf8_lossy(&output.stdout)
                        );
                    }
                    Err(e) => println!("NUKE SHIDDED {}", e),
                }
            });
        }
        false => {}
    }
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
    fn test_fuzz() {
        let url = "google.com";
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            super::fuzzing::fuzz(url.to_string()).await;
        });
    }
}
