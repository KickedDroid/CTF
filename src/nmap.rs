use std::process::Command;



pub(crate) async fn nmap_scan(ip: String) {

    println!("Running nmap on {}", ip.clone());
   
    let nmap_result = Command::new("nmap")
        .args([
            ip.clone().as_str(),
            "-sC",
            "-Pn",
            "-sV",
            ])
        .output()
        .expect("nmap scan failed...");
    print!("{}", String::from_utf8_lossy(&nmap_result.stdout));
}

fn process_nmap_result(res: String)  {
    match res.as_str() {
        res if res.contains("5432") => todo!(),
        res if res.contains("53") => todo!(),
        res if res.contains("80") => {
            println!("http 80 service found..")
        },
        res if res.contains("443") => todo!(),
        res if res.contains("21") => {
            println!("Found FTP..")
        },
        _ => {},
    } 
}