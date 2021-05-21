# webview-app
Simple integration of a web view in an application window like Electron, but using rust as programming language
## WARNING
This is just an early start!

## Prerequisites on Linux (Fedora)
* ```sudo dnf install gtk3-devel```
* ```sudo dnf install libsoup-devel```
* ```sudo dnf install webkit2gtk3-devel.x86_64```
* ```sudo dnf install libudev-devel```

## To run example
```cargo run --example hello```

## Icon in Windows
Use ```winres``` and build-script ```build.rs``` with ```res.set_icon()```. This Icon will be the web viw windows icon.

