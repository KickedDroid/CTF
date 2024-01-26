use std::process::Command;


pub(crate) fn subdomain_enum(url: String) {
    let subdomain_enum_cmd = format!("./subscout -t {}", url);
    let subd_enum_result = Command::new("bash")
        .arg("-c")
        .arg(subdomain_enum_cmd)
        .output()
        .expect("Subdomain Enumeration failed...");
    if subd_enum_result.status.success() {
        println!("{}",String::from_utf8_lossy(&subd_enum_result.stdout));
    }
}