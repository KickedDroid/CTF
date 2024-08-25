use std::{io::BufRead, process::Command};

use clap::builder::Str;


pub(crate) async fn whowhat(url: String) {
    let pb = indicatif::ProgressBar::new(100);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}").unwrap()
            .progress_chars("#>-"),
    );
    
    let whowhat_result = Command::new("whatweb")
        .args([
            url.clone().as_str(),
            "-a1"
        ])
        .output()
        .expect("WhoWhat Enumeration failed...");
    let res = String::from_utf8_lossy(&whowhat_result.stdout);
    let lines: Vec<&str> = res.split(", ").collect();
    println!("WhatWeb Results ---------------------------------");
    for line in lines.clone() {
        println!("{}", line);
    }
    pb.finish();
    println!("End of WhatWeb Results --------------------------\n");
}