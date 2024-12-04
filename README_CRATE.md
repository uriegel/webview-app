# webview-app
Rust Web View Application for Windows and Linux similar to Electron, but very light weight. It offers the possibility to make web requests from the web site to the rust app and to send events from rust to the web site. The web site can be hosted as integrated resource, of course alternatively via HTTP(s):// or file://.

Sample webview_app:
![Sample WebView app](readme/sampleapp.png)

## Version >= 1.0.0
webview_app >= version 1.0.0 is completely redesigned and more light weight, it doesn't include a warp server anymore and doesn't need the tokio runtime either.

If you want to update from older version, you have to migrate, please look at this documentation or this [tutorial](https://uriegel.github.io/webview-app/doc/tutorial/).

If you don't want to migrate, you have to stick to the old version 0.5.1. Here is the [crate documentation for Version 0.5.1](https://docs.rs/webview_app/0.5.1/webview_app/).

## Features <a name="features"></a>

webview_app includes following features:
* Functional approach with a builder pattern
* (almost) the same setup for Windows and Linux version
* Uses WebView2 on Windows and WebKitGtk-6.0 on Linux
* Can serve the web site via resources (single file approach)
* Optional save and restore of window bounds
* Has an integrated event sink mechanismn, so you can retrieve javascript events from the Rust app
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

