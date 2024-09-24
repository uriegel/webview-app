# webview-app

## Prerequisites on Linux (Fedora)
* ```sudo dnf install gtk4-devel```

## Run examples

```cargo run --example hello```

oder 

```cargo run --example hello --release```

# O L D 
## Prerequisites on Linux (Fedora)
* ```sudo dnf install libsoup-devel```
* ```sudo dnf install webkit2gtk3-devel.x86_64```
* ```sudo dnf install libudev-devel```

## Icon in Windows
Use ```winres``` and build-script ```build.rs``` with ```res.set_icon()```. This Icon will be the web view windows icon.

