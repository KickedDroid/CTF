use std::process::Command;

pub(crate) fn fuzz(url: String) {
    println!("Running Feroxbuster on {}", url.clone());
    
    let ferox_result = Command::new("feroxbuster")
        .args([
            "--url",
            url.clone().as_str(),
            "-w",
            "../wordlists/raft-medium-directories.txt",
            "-o",
            format!("/ferox/ferox-{}.txt", url.clone()).as_str(),
        ])
        .output()
        .expect("feroxbuster scan failed...");

    print!("{}", String::from_utf8_lossy(&ferox_result.stdout));
}