# webview-app

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

