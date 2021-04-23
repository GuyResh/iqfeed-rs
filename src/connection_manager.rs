use std::process::Command;

pub fn launch(product_id: &str, version: &str, path: &str, username: &str, password: &str) {
    if cfg!(windows) {
        windows(product_id, version, path, username, password);
    } else if cfg!(unix) {
        unix(product_id, version, path, username, password);
    }
}

fn windows(product_id: &str, version: &str, path: &str, username: &str, password: &str) {
    Command::new(path)
        .args(&["-product", product_id])
        .args(&["-version", version])
        .args(&["-login", username])
        .args(&["-password", password])
        .args(&["-autoconnect", "-saveloginingo"])
        .spawn()
        .unwrap();
}

fn unix(product_id: &str, version: &str, path: &str, username: &str, password: &str) {
    Command::new("xvfb-run")
        .args(&["-s", "-noreset", "a"])
        .arg(path)
        .args(&["-product", product_id])
        .args(&["-version", version])
        .args(&["-login", username])
        .args(&["-password", password])
        .args(&["-autoconnect", "-saveloginingo"])
        .spawn()
        .unwrap();
}
