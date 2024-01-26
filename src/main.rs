use std::env;
use std::process::{Command, Output, Stdio};
use subrut::models::error::Error;
#[tokio::main]
async fn main() ->  Result<(), Error> {
    display_name();
    let args: Vec<String> = env::args().collect();
    let ip = &args[1];

    println!("Running scans on {:?} using nmap and subscout....", ip);

    let nmap_cmd = format!("nmap -sV -sC {:?}", ip);
    let nmap_result = Command::new("bash")
        .arg("-c")
        .arg(nmap_cmd)
        .output()
        .expect("nmap scan failed...");
    if nmap_result.status.success() {
        
        println!("{}",String::from_utf8_lossy(&nmap_result.stdout));
        process_nmap_result(String::from_utf8_lossy(&nmap_result.stdout).to_string())
    }

    let subdomain_enum_cmd = format!("./subscout -t {}", ip);
    let subd_enum_result = Command::new("bash")
        .arg("-c")
        .arg(subdomain_enum_cmd)
        .output()
        .expect("Subdomain Enumeration failed...");
    if subd_enum_result.status.success() {
        println!("{}",String::from_utf8_lossy(&subd_enum_result.stdout));
        process_nmap_result(String::from_utf8_lossy(&subd_enum_result.stdout).to_string())
    }
    Ok(())
}

fn display_name() {
    println!(
        "
        ██████╗░███████╗███╗░░██╗████████╗███████╗░██████╗████████╗
        ██╔══██╗██╔════╝████╗░██║╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝
        ██████╔╝█████╗░░██╔██╗██║░░░██║░░░█████╗░░╚█████╗░░░░██║░░░
        ██╔═══╝░██╔══╝░░██║╚████║░░░██║░░░██╔══╝░░░╚═══██╗░░░██║░░░
        ██║░░░░░███████╗██║░╚███║░░░██║░░░███████╗██████╔╝░░░██║░░░
        ╚═╝░░░░░╚══════╝╚═╝░░╚══╝░░░╚═╝░░░╚══════╝╚═════╝░░░░╚═╝░░░
        
        ░█████╗░██╗░░░██╗████████╗░█████╗░███╗░░░███╗░█████╗░████████╗██╗░█████╗░███╗░░██╗  ░██╗░░░░░░░██╗██╗████████╗██╗░░██╗
        ██╔══██╗██║░░░██║╚══██╔══╝██╔══██╗████╗░████║██╔══██╗╚══██╔══╝██║██╔══██╗████╗░██║  ░██║░░██╗░░██║██║╚══██╔══╝██║░░██║
        ███████║██║░░░██║░░░██║░░░██║░░██║██╔████╔██║███████║░░░██║░░░██║██║░░██║██╔██╗██║  ░╚██╗████╗██╔╝██║░░░██║░░░███████║
        ██╔══██║██║░░░██║░░░██║░░░██║░░██║██║╚██╔╝██║██╔══██║░░░██║░░░██║██║░░██║██║╚████║  ░░████╔═████║░██║░░░██║░░░██╔══██║
        ██║░░██║╚██████╔╝░░░██║░░░╚█████╔╝██║░╚═╝░██║██║░░██║░░░██║░░░██║╚█████╔╝██║░╚███║  ░░╚██╔╝░╚██╔╝░██║░░░██║░░░██║░░██║
        ╚═╝░░╚═╝░╚═════╝░░░░╚═╝░░░░╚════╝░╚═╝░░░░░╚═╝╚═╝░░╚═╝░░░╚═╝░░░╚═╝░╚════╝░╚═╝░░╚══╝  ░░░╚═╝░░░╚═╝░░╚═╝░░░╚═╝░░░╚═╝░░╚═╝
        
        ██████╗░██╗░░░██╗░██████╗████████╗
        ██╔══██╗██║░░░██║██╔════╝╚══██╔══╝
        ██████╔╝██║░░░██║╚█████╗░░░░██║░░░
        ██╔══██╗██║░░░██║░╚═══██╗░░░██║░░░
        ██║░░██║╚██████╔╝██████╔╝░░░██║░░░
        ╚═╝░░╚═╝░╚═════╝░╚═════╝░░░░╚═╝░░░"
    );
}

fn process_nmap_result(res: String)  {
    if res.contains("5432/tcp open  postgresql") {
        println!("Running Postgresql Enumeration")
    }
    if res.contains("pat") {

    }
}