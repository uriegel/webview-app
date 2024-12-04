# webview-app
Rust Web View Application for Windows and Linux similar to Electron, but very light weight. It offers the possibility to make web requests from the web site to the rust app and to send events from rust to the web site. The web site can be hosted as integrated resource, of course alternatively via HTTP(s):// or file://.

Sample webview_app:
![Sample WebView app](readme/sampleapp.png)

webview_app >= version 1.0.0 is completely redesigned and it doesn't include a warp server anymore.

## Features <a name="features"></a>

webview_app includes following features:
* Functional approach with a builder pattern
* (almost) the same setup for Windows and Linux version
* Uses WebView2 on Windows and WebKitGtk-6.0 on Linux
* Can serve the web site via resources (single file approach)
* Optional save and restore of window bounds
* Has an integrated event sink mechanismen, so you can retrieve javascript events from the Rust app
* Offers the possibility to serve requests from javascript to Rust
* You can expand the Gtk4 Window (on Linux) with a custom header bar
* You can alternatively disable the Windows titlebar and borders, and you can build a title bar in HTML with standard Windows logic for closing, maximizing, restoring resizing, snap to dock, ...

Functional approach with webview builder:

```rs
fn on_activate(app: &Application)->WebView {
    let webview = WebView::builder(app)
        .title("Website form custom resources 🦞")
        .save_bounds()
        .devtools(true)
        .webroot(include_dir!("webroots/custom_resources"))
        .query_string("?param1=123&param2=456")
        .default_contextmenu_disabled()
        .build();

     webview
}

fn main() {
    Application::new("de.uriegel.hello")
    .on_activate(on_activate)
    .run();
}
```
Sample of a Windows App with custom titlebar:
![custom titlebar](readme/customTitlebar.png) 


## Prerequisites on Linux (Fedora)
* ```sudo dnf install gtk4-devel```
* ```sudo dnf install webkitgtk6.0-devel```
* ```sudo dnf install libadwaita-devel```

## Prerequisites on Linux (Ubuntu)
* ```sudo apt install libgtk-4-dev```
* ```sudo apt install libwebkitgtk-6.0-dev```
* ```sudo apt install libadwaita-1-dev```

## Run examples

```cargo run --example hello```

oder 

```cargo run --example hello --release```

## Icon in Windows
Use ```winres``` and build-script ```build.rs``` with ```res.set_icon()```. This Icon will be the web view windows icon.

## Build doc

On windows:

``` cargo doc --target x86_64-pc-windows-msvc --no-deps```

On Linux

``` cargo doc --target x86_64-unknown-linux-gnu --no-deps```

Branch ```gh-pages``` created

Cloned to local repository

All files deleted

Content of ```C:\Projekte\webview-app\target\x86_64-pc-windows-msvc\doc``` copies to .\doc\windows

Content of ```C:\Projekte\webview-app\target\x86_64-unknown-linux-gnu\doc``` copies to .\doc\linux

Commited changes