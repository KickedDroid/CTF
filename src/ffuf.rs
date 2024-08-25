use std::process::Command;
use std::collections::HashMap;
use serde_json::Value;

pub async fn run_ffuf_with_smart_filter(domain: &str) -> Result<(), Box<dyn std::error::Error>> {
    let wordlist = "/home/nacho/Documents/tools/SecLists/Discovery/DNS/subdomains-top1million-5000.txt";
    
    // Step 1: Run a sample of requests
    println!("Running initial sample of ffuf requests...");
    let sample_output = Command::new("ffuf")
        .args(&[
            "-w", &format!("{}:FUZZ", wordlist),
            "-u", &format!("http://{}", domain),
            "-H", &format!("Host: FUZZ.{}", domain),
            "-c",
            "-p", "0.1", // Use 10% of the wordlist as a sample
            "-of", "json"
        ]).output()?;

    // Step 2: Analyze the sample results
    let sample_results: Value = serde_json::from_str(&String::from_utf8_lossy(&sample_output.stdout))?;
    println!("{}", sample_results);
    let results = sample_results["results"].as_array().unwrap();

    let mut response_characteristics = HashMap::new();

    for result in results {
        let key = (
            result["status"].as_u64().unwrap(),
            result["length"].as_u64().unwrap(),
            result["words"].as_u64().unwrap(),
            result["lines"].as_u64().unwrap(),
        );
        *response_characteristics.entry(key).or_insert(0) += 1;
    }

    // Find the most common response characteristic
    let most_common = response_characteristics.iter()
        .max_by_key(|&(_, count)| count)
        .map(|(key, _)| key)
        .unwrap();

    // Step 3: Run the main ffuf scan with filters
    println!("Running main ffuf scan with smart filters...");
    let main_output = Command::new("ffuf")
        .args(&[
            "-w", &format!("{}:FUZZ", wordlist),
            "-u", &format!("http://{}", domain),
            "-H", &format!("Host: FUZZ.{}", domain),
            "-fc", &most_common.0.to_string(),
            "-fs", &most_common.1.to_string(),
            "-fw", &most_common.2.to_string(),
            "-fl", &most_common.3.to_string(),
            "-c",
            "-o", "ffuf_results.json",
            "-of", "json"
        ])
        .output()?;

    println!("ffuf scan completed. Results saved in ffuf_results.json");
    println!("Used filters: -fc {} -fs {} -fw {} -fl {}", 
             most_common.0, most_common.1, most_common.2, most_common.3);

    Ok(())
}