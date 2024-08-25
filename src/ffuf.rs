use std::process::Command;
use std::collections::HashMap;
use serde_json::Value;

pub async fn run_ffuf_with_auto_filter(domain: &str) -> Result<(), Box<dyn std::error::Error>> {
    let wordlist = "/home/nacho/Documents/tools/SecLists/Discovery/DNS/subdomains-top1million-5000.txt";
    
    // Step 1: Run a sample of requests
    println!("Running initial sample of ffuf requests...");
    let sample_output = Command::new("ffuf")
        .args(&[
            "-w", &format!("{}:FUZZ", wordlist),
            "-u", &format!("http://{}", domain),
            "-H", &format!("Host: FUZZ.{}", domain),
            "-c",
            "-t", "50",
            "-p", "0.1", // Use 10% of the wordlist as a sample
            "-o", "sample_results.json",
            "-of", "json"
        ])
        .output()?;

    // Step 2: Analyze the sample results
    let sample_results: Value = serde_json::from_str(&String::from_utf8_lossy(&sample_output.stdout))?;
    let results = sample_results["results"].as_array().unwrap();

    let mut sizes = Vec::new();
    let mut status_codes = HashMap::new();

    for result in results {
        sizes.push(result["length"].as_u64().unwrap());
        let status = result["status"].as_u64().unwrap();
        *status_codes.entry(status).or_insert(0) += 1;
    }

    // Calculate average size and most common status code
    let avg_size: u64 = sizes.iter().sum::<u64>() / sizes.len() as u64;
    let most_common_status = status_codes.iter().max_by_key(|&(_, count)| count).map(|(code, _)| code).unwrap();

    // Step 3: Run the main ffuf scan with filters
    println!("Running main ffuf scan with auto-generated filters...");
    let main_output = Command::new("ffuf")
        .args(&[
            "-w", &format!("{}:FUZZ", wordlist),
            "-u", &format!("http://{}", domain),
            "-H", &format!("Host: FUZZ.{}", domain),
            "-fs", &format!("{}", avg_size), // Filter out responses with the average size
            "-fc", &format!("{}", most_common_status), // Filter out the most common status code
            "-c",
            "-o", "ffuf_results.json",
            "-of", "json"
        ])
        .output()?;

    println!("ffuf scan completed. Results saved in ffuf_results.json");
    println!("Used filters: -fs {} -fc {}", avg_size, most_common_status);
    println!("{}", String::from_utf8_lossy(&main_output.stdout));
    Ok(())
}