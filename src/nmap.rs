use std::process::Command;



pub(crate) fn nmap_scan(ip: String) -> String {
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
    return String::from_utf8_lossy(&nmap_result.stdout.clone()).to_string()
}

fn process_nmap_result(res: String)  {
    if res.contains("5432/tcp open  postgresql") {
        println!("Running Postgresql Enumeration")
    }
    if res.contains("pat") {

    }
}