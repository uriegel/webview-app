# webview-app

# O L D 
## Prerequisites on Linux (Fedora)
* ```sudo dnf install gtk3-devel```
* ```sudo dnf install libsoup-devel```
* ```sudo dnf install webkit2gtk3-devel.x86_64```
* ```sudo dnf install libudev-devel```

## To run example
```cargo run --example hello```

## Icon in Windows
Use ```winres``` and build-script ```build.rs``` with ```res.set_icon()```. This Icon will be the web view windows icon.

