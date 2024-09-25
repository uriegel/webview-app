# webview-app

## Prerequisites on Linux (Fedora)
* ```sudo dnf install gtk4-devel```
* ```sudo dnf install webkitgtk6.0-devel```

## Run examples

```cargo run --example hello```

oder 

```cargo run --example hello --release```

## Icon in Windows
Use ```winres``` and build-script ```build.rs``` with ```res.set_icon()```. This Icon will be the web view windows icon.

