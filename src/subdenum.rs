use std::process::Command;


pub(crate) async fn subdomain_enum(url: String) {
    let subdomain_enum_cmd = format!("./subscout -t {}", url.clone());
    let subd_enum_result = Command::new("./subscoutb")
        .args([
            "-t",
            url.clone().as_str(),
        ])
        .output()
        .expect("Subdomain Enumeration failed...");
    print!("{}", String::from_utf8_lossy(&subd_enum_result.stdout));
}