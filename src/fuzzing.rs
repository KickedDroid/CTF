use std::process::Command;

pub(crate) fn fuzz(url: String) -> String {
    println!("Running Feroxbuster on {}", url.clone());
    let ferox_cmd = format!("feroxbuster --url {:?} -w ../wordlists/raft-medium-directories.txt -o output/ferox-{}.txt", url.clone(), url.clone());
    let ferox_result = Command::new("bash")
        .arg("-c")
        .arg(ferox_cmd)
        .output()
        .expect("feroxbuster scan failed...");
    if ferox_result.status.success() {
        println!("Completed Ferroxbuster!! See output file in /output");
    } else {
        println!("Failed Feroxbuster")
    }
    return String::from_utf8_lossy(&ferox_result.stdout.clone()).to_string()
}