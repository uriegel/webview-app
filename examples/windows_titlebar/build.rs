fn main() {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("app.ico")
       .set_version_info(winresource::VersionInfo::PRODUCTVERSION, 0x0001000000000000);
    res.compile().unwrap();    
}