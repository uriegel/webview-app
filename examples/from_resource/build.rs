#[cfg(target_os = "windows")]
extern crate winres;

#[cfg(target_os = "windows")]
fn main() {
    let mut res = winres::WindowsResource::new();
    // res.set_icon("resources/app.ico");
    // res.write_resource_file("resources/win.rc");
    res.set_resource_file("resources/win.rc");
    res.compile().unwrap();

}

#[cfg(target_os = "linux")]
fn main() {}