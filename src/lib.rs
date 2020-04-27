use std::process::Command;
extern crate winapi;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
pub fn launch_shortcut(shortcut: String) -> PyResult<bool> {
    println!("Attempting to launch: {}", shortcut);
    let _child = Command::new("cmd.exe")
        .arg("/C")
        .arg(shortcut)
        .spawn()
        .expect("failed to launch program");

    return Ok(true);
}

#[pyfunction]
pub fn launch_web(browser: String, url: String, new: bool) -> PyResult<bool> {
    println!("Attempting to launch: {}", url);

    let _child = Command::new("cmd.exe")
        .arg("/C")
        .arg("start")
        .arg(browser)
        .arg(if new == true { "--new-browser" } else { "" })
        .arg(url)
        .spawn()
        .expect("failed to launch browser");

    return Ok(true);
}

#[pymodule]
fn launcher(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(launch_shortcut))?;
    m.add_wrapped(wrap_pyfunction!(launch_web))?;
    Ok(())
}
