use reqwest;
use serde_json::{Value};
use std::{default, env, thread};
use std::time::Duration;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ZapConfig {
    pub zap_api_url: String,
    pub api_key: String,
}

impl ZapConfig {
    pub fn new() -> Self {
        let conf = ZapConfig::from_env().expect("Need to add API key to env");
        return conf;
    }

    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let zap_api_url = env::var("ZAP_API_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
        let api_key = env::var("ZAP_API_KEY").expect("ZAP_API_KEY not set in environment");

        Ok(ZapConfig {
            zap_api_url,
            api_key,
        })
    }
}

pub struct ZapScanner {
    config: ZapConfig,
}


impl ZapScanner {
    pub fn new(zap_api_url: String, api_key: String) -> Self {
        ZapScanner {
           config: ZapConfig {
                zap_api_url: zap_api_url,
                api_key: api_key,
            }
        }
    }

    pub async fn scan_target(&self, target_url: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!("Starting ZAP scan on: {}", target_url);

        // Start Spider
        println!("Starting Spider...");
        let spider_scan_id = self.start_spider(target_url).await?;
        self.wait_for_spider_completion(&spider_scan_id).await?;
        println!("Spider scan completed.");

        // Start Active Scan
        println!("Starting Active Scan...");
        let active_scan_id = self.start_active_scan(target_url).await?;
        self.wait_for_active_scan_completion(&active_scan_id).await?;
        println!("Active scan completed.");

        // Get Alerts
        let alerts = self.get_alerts(target_url).await?;
        Ok(alerts)
    }

    async fn start_spider(&self, target_url: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/JSON/spider/action/scan/?apikey={}&url={}", self.config.zap_api_url, self.config.api_key, target_url);
        let response: Value = reqwest::get(&url).await?.json().await?;
        Ok(response["scan"].as_str().unwrap_or("").to_string())
    }

    async fn wait_for_spider_completion(&self, scan_id: &str) -> Result<(), reqwest::Error> {
        loop {
            let url = format!("{}/JSON/spider/view/status/?apikey={}&scanId={}", self.config.zap_api_url, self.config.api_key, scan_id);
            let response: Value = reqwest::get(&url).await?.json().await?;
            let status = response["status"].as_str().unwrap_or("0");
            if status == "100" {
                break;
            }
            thread::sleep(Duration::from_secs(5));
        }
        Ok(())
    }

    async fn start_active_scan(&self, target_url: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/JSON/ascan/action/scan/?apikey={}&url={}", self.config.zap_api_url, self.config.api_key, target_url);
        let response: Value = reqwest::get(&url).await?.json().await?;
        Ok(response["scan"].as_str().unwrap_or("").to_string())
    }

    async fn wait_for_active_scan_completion(&self, scan_id: &str) -> Result<(), reqwest::Error> {
        loop {
            let url = format!("{}/JSON/ascan/view/status/?apikey={}&scanId={}", self.config.zap_api_url, self.config.api_key, scan_id);
            let response: Value = reqwest::get(&url).await?.json().await?;
            let status = response["status"].as_str().unwrap_or("0");
            if status == "100" {
                break;
            }
            thread::sleep(Duration::from_secs(5));
        }
        Ok(())
    }

    async fn get_alerts(&self, target_url: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/JSON/core/view/alerts/?apikey={}&baseurl={}", self.config.zap_api_url, self.config.api_key, target_url);

        let response: Value = reqwest::get(&url).await?.json().await?;
        Ok(serde_json::to_string_pretty(&response).unwrap())
    }
}