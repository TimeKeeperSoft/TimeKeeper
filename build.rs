#[cfg(windows)]
use winresource;

#[cfg(windows)]
fn main() {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("./assets/logo.ico");
    res.compile().unwrap();

    slint_build::compile("ui/main-win.slint").expect("Slint build failed");
}

#[cfg(unix)]
fn main() {}
