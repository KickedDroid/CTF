use std::process::Command;

pub(crate) async fn fuzz(url: String) {
    println!("Running Feroxbuster on {}", url.clone());
    
    let ferox_result = Command::new("feroxbuster")
        .args([
            "--url",
            url.clone().as_str(),
            "-w",
            "wordlists/raft-medium-directories.txt",
        ])
        .output()
        .expect("feroxbuster scan failed...");
    println!("Feroxbuster Results ---------------------------------");
    print!("{}", String::from_utf8_lossy(&ferox_result.stdout));
    println!("End of Feroxbuster Results --------------------------\n")
}