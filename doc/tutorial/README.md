# webview-app
Rust Web View Application for Windows and Linux similar to Electron. It offers the possibility to make web requests from the web site to the rust app and to send events from rust to the web site. The web site can be hosted as integrated resource, of course alternatively via HTTP(s):// or file://.

Sample webview_app:
![Sample WebView app](sampleapp.png)

# Table of contents
1. [Introduction](#features)
2. [Setup](#setup)
    1. [Additional step for Windows](#prewindows)
3. [Hello World (a minimal web view app)](#helloworld)    

## Features <a name="features"></a>

webview_app includes following features:
* Functional approach with a builder pattern
* The same setup (almost) for Windows and Linux version 
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
![custom titlebar](customTitlebar.png) 

## Setup <a name="setup"></a>

In order to create a WebView app, you have to  create a rust console app with ```cargo new``` or ```cargo init```.

Then add the webview_app crate with ```cargo add webview_app```.

## Setup <a name="setup"></a>
### Additional step for Windows <a name="prewindows"></a>
To get rid of the console window, you can hide it.

Add the following code to your main.rs file at the top:

```rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Allows console to show up in debug build but not release build.
```

Now there is no console window in release mode but not debug mode to be able to see console logs.

## Hello World (a minimal web view app) <a name="helloworld"></a>

![Sample WebView app](helloworld.png)