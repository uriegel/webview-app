pub struct RequestData<'a> {
    pub cmd: &'a str,
    pub id: &'a str,
    pub json: &'a str
}

impl <'a>RequestData<'a> {
    pub fn new(msg: &'a str)->RequestData<'a> {
        let msg = &msg[8..];
        let idx = msg.find(',').unwrap();
        let 
        cmd = &msg[..idx];
        let msg= &msg[idx+1..];
        let idx = msg.find(',').unwrap();
        let id = &msg[..idx];
        let json = &msg[idx+1..];
        let _ = &json[1..2];
        RequestData {
            cmd,
            id,
            json
        }
    }
}

pub fn get(no_native_titlebar: bool, title: &str, windows: bool, files_drop: bool)->String {
    format!(
r##"
{}
{}

var WebView = (() => {{
    {}
    {}
    {}

    let evtHandler = () => {{ }}
    const setDroppedFilesEventHandler = eh => evtHandler = eh

    const setDroppedEvent = success => evtHandler(success)

    initializeNoTitlebar = () => WEBVIEWNoNativeTitlebarInitialize()

    closeWindow = () => window.close()

    return {{
        initializeNoTitlebar,
        showDevTools,
        startDragFiles,
        request,
        dropFiles,
        filesDropped,
        setDroppedFilesEventHandler,
        setDroppedEvent,
        closeWindow,
        backtothefuture,
        additionalObjectsBack,
        startDragFilesBack
    }}
}})()

try {{
    if (onWebViewLoaded) 
        onWebViewLoaded()
}} catch {{ }}"##, no_titlebar_script(no_native_titlebar, title), request_result(windows), platform_specifics(windows), 
                requests(), on_files_drop(files_drop))
}

fn platform_specifics(windows: bool)->String { 
    if windows {
r##"        
    const showDevTools = () => window.chrome.webview.postMessage("devtools")
    let startDragFilesBackRes = null
    const startDragFiles = files => {{
        return new Promise(res => {{
            window.chrome.webview.postMessage('startDragFiles,' + JSON.stringify(files))
            startDragFilesBackRes = res
        }})
    }}
    function startDragFilesBack() {{
        if (startDragFilesBackRes) {{
            startDragFilesBackRes()
            startDragFilesBackRes = null
        }}
    }}
    let additionalObjectsBackRes = null
    function filesDropped(dataTransfer) {{
        return new Promise(res => {{
            chrome.webview.postMessageWithAdditionalObjects("AdditionalObjects", dataTransfer.files)
            additionalObjectsBackRes = res
        }})
    }}
    function additionalObjectsBack(files) {{
        if (additionalObjectsBackRes) {{
            additionalObjectsBackRes(files)
            additionalObjectsBackRes = null
        }}
    }}
"##
    } else {
r##"                
    const showDevTools = () => fetch('req://showDevTools')
    const startDragFiles = files => fetch('req://startDragFiles', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ files })
    })
    async function filesDropped(dataTransfer) {{}}
"##
    }.to_string()
}

fn no_titlebar_script(no_native_titlebar: bool, title: &str)->String {
    if no_native_titlebar {
        format!(
r##"        
function WEBVIEWsetMaximized(m) {{
    const maximize = document.getElementById('$MAXIMIZE$')
    if (maximize)
        maximize.style.display = m ? 'none' : ''

    const restore = document.getElementById('$RESTORE$')
    if (restore)
        restore.style.display = m ? '' : 'none'
}}

const WEBVIEWNoNativeTitlebarInitialize = () => {{

    const dragRegion = document.getElementById('$DRAG_REGION$')
    if (dragRegion) {{
        dragRegion.style.setProperty('-webkit-app-region', 'drag')
        let activeElement = null
        dragRegion.onmousedown = e => {{
            activeElement = document.activeElement
        }}
        dragRegion.onmouseup = e => activeElement.focus()
    }}

    const title = document.getElementById('$TITLE$')
    if (title)
        title.innerText = '{}'
    const close = document.getElementById('$CLOSE$')
    if (close)
        close.onclick = () => window.close()
    const maximize = document.getElementById('$MAXIMIZE$')
    if (maximize) {{
        maximize.onclick = () => {{
            window.chrome.webview.postMessage("MaximizeWindow")
            WEBVIEWsetMaximized(true)
        }}
    }}
    const minimize = document.getElementById('$MINIMIZE$')
    if (minimize)
        minimize.onclick = () => window.chrome.webview.postMessage("MinimizeWindow")
    const restore = document.getElementById('$RESTORE$')
    if (restore) {{
        restore.onclick = () => {{
            window.chrome.webview.postMessage("RestoreWindow")
            WEBVIEWsetMaximized(false)
        }}
        restore.style.display = 'none'
    }}
}}
WEBVIEWNoNativeTitlebarInitialize()
        "##, title).to_string()
    } else { "".to_string() }
}

fn on_files_drop(files_drop: bool)->String {
    if files_drop {
r##"
    function dropFiles(id, move, droppedFiles) {{
        chrome.webview.postMessageWithAdditionalObjects({{
            msg: 1,
            text: id,
            move
        }}, droppedFiles)
    }}
"##
    } else { 
r##"   
    function dropFiles() {}
"## 
    }.to_string()
}

fn requests()->String {
r##"        
    var webviewrequestsid = 0
    var webviewrequests = new Map()

    const backtothefuture = (data) => {
        if (data.startsWith("result,")) {
            const msg = data.substring(7)
            const idx = msg.indexOf(',')
            const id = msg.substring(0, idx)
            const back = msg.substring(idx + 1).replace("u0027", "'")
            const json = JSON.parse(back)
            const res = webviewrequests.get(id)    
            webviewrequests.delete(id)
            res(json)
        }
        else
            console.log("Message received", data)
    }
    
    const request = (method, data) => new Promise(res => {
        webviewrequests.set((++webviewrequestsid).toString(), res)
        const msg = `request,${method},${webviewrequestsid},${JSON.stringify(data)}`
        send_request(msg)
    })
"##.to_string()
}

fn request_result(windows: bool)->String {
    if windows {
r##"    
    function send_request(data) {
        window.chrome.webview.postMessage(data)
    }

    window.chrome.webview.addEventListener('message', arg => {
        WebView.backtothefuture(arg.data) 
    })
"##
    }  else { 
r##"            
    function send_request(data) {
        alert(data)
    }
"##
     }.to_string() 
}