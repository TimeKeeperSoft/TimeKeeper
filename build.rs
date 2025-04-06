#[cfg(windows)]
use winresource;

#[cfg(windows)]
fn main() {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("./assets/logo.ico");
    res.compile().unwrap();
}

#[cfg(unix)]
fn main() {}
