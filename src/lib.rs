use std::process::Command;
extern crate winapi;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn launch_shortcut(shortcut: String) -> PyResult<bool> {
    println!("Attempting to launch: {}", shortcut);
    let _child = Command::new("cmd.exe")
        .arg("/C")
        .arg(shortcut)
        .spawn()
        .expect("failed to launch program");

    return Ok(true);
}

#[pyfunction]
fn launch_web(browser: String, url: String) -> PyResult<bool> {
    println!("Attempting to launch: {}", url);

    let _child = Command::new("cmd.exe")
        .arg("/C")
        .arg("start")
        .arg("")
        .arg(browser)
        .arg("")
        .arg(url)
        .spawn()
        .expect("failed to launch browser");

    return Ok(true);
}

#[pymodule]
fn Launcher(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(launch_shortcut))?;
    m.add_wrapped(wrap_pyfunction!(launch_web))?;
    Ok(())
}
