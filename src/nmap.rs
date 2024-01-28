use std::process::Command;



pub(crate) fn nmap_scan(ip: String) -> String {
    let nmap_cmd = format!("nmap -sV -sC {:?}", ip);
    let nmap_result = Command::new("bash")
        .arg("-c")
        .arg(nmap_cmd)
        .output()
        .expect("nmap scan failed...");
    if nmap_result.status.success() {
        //println!("{}",String::from_utf8_lossy(&nmap_result.stdout));
        process_nmap_result(String::from_utf8_lossy(&nmap_result.stdout).to_string())
    }
    return String::from_utf8_lossy(&nmap_result.stdout.clone()).to_string()
}

fn process_nmap_result(res: String)  {
    match res.as_str() {
        res if res.contains("5432/postgres  open") => todo!(),
        res if res.contains("53/tcp  open domain") => todo!(),
        res if res.contains("80/tcp  open   http") => {
            println!("http 80 service found..")
        },
        res if res.contains("443/tcp open   ssl/http") => todo!(),
        res if res.contains("21/ftp  open") => {
            println!("Found FTP..")
        },
        _ => {},
    } 
}