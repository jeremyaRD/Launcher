use std::process::Command;
extern crate winapi;

fn main() {
    // println!("Attempting to launch: {}");
    test_shortcut();
    // test_web();
}

fn test_web() {
    let _child = Command::new("cmd.exe")
        .arg("/C")
        .arg("start")
        .arg("chrome")
        .arg("--new-browser")
        .arg("google.ca")
        .spawn()
        .expect("failed to launch browser");
}

fn test_shortcut() {
    let _child = Command::new("cmd.exe")
        .arg("/C")
        .arg("C:\\Users\\Jeremy\\Desktop\\GitHub Desktop.lnk")
        .spawn()
        .expect("failed to launch program");
}
