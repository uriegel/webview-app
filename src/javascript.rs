pub fn get(no_native_titlebar: bool, title: &str, port: i32, windows: bool, files_drop: bool)->String {
    format!(
r##"
{}

var webViewEventSinks = new Map()

var WebView = (() => {{
    {}
    {}
    {}
    const request = async (method, data) => {{
        const res = await fetch(`http://localhost:{}/requests/${{method}}`, {{
            method: 'POST',
            headers: {{ 'Content-Type': 'application/json' }},
            body: JSON.stringify(data)
        }})
        return await res.json()
    }}

    const registerEvents = (id, evt) => {{
        webViewEventSinks.set(id, evt)
        onEventsCreated(id)
    }}

    let evtHandler = () => {{ }}
    const setDroppedFilesEventHandler = eh => evtHandler = eh

    const setDroppedEvent = success => evtHandler(success)

    initializeNoTitlebar = () => WEBVIEWNoNativeTitlebarInitialize()

    getRequestUrl = () => `http://localhost:{}/requests/`

    closeWindow = () => window.close()

    return {{
        initializeNoTitlebar,
        showDevTools,
        startDragFiles,
        request,
        registerEvents,
        dropFiles,
        setDroppedFilesEventHandler,
        setDroppedEvent,
        getRequestUrl,
        closeWindow
    }}
}})()

try {{
    if (onWebViewLoaded) 
        onWebViewLoaded()
}} catch {{ }}"##, no_titlebar_script(no_native_titlebar, title), dev_tools(windows), on_files_drop(files_drop), on_events_created(windows), port, port)
}

fn dev_tools(windows: bool)->String { 
    if windows {
    // TODO startDragFiles in devtools?
r##"        
    const showDevTools = () => callback.ShowDevtools()
    const startDragFiles = files => callback.StartDragFiles(JSON.stringify({ files }))
"##
    } else {
r##"                
    const showDevTools = () => fetch('req://showDevTools')
    const startDragFiles = files => fetch('req://startDragFiles', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ files })
    })
"##
    }.to_string()
}

fn on_events_created(windows: bool)->String {
    if windows { "const onEventsCreated = id => callback.OnEvents(id)" } 
    else { "const onEventsCreated = id => fetch(`req://onEvents/${id}`)" }.to_string() 
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

    const favicon = document.getElementById('$FAVICON$')
    if (favicon)
        favicon.src = 'res://icon'
    const title = document.getElementById('$TITLE$')
    if (title)
        title.innerText = '{}'
    const close = document.getElementById('$CLOSE$')
    if (close)
        close.onclick = () => window.close()
    const maximize = document.getElementById('$MAXIMIZE$')
    if (maximize) 
        maximize.onclick = () => callback.MaximizeWindow()
    const minimize = document.getElementById('$MINIMIZE$')
    if (minimize)
        minimize.onclick = () => callback.MinimizeWindow()
    const restore = document.getElementById('$RESTORE$')
    if (restore) {{
        restore.onclick = () => callback.RestoreWindow()
        restore.style.display = 'none'
    }}
    const hamburger = document.getElementById('$HAMBURGER$')
    if (hamburger) 
        hamburger.onclick = () => callback.OnHamburger(hamburger.offsetLeft / document.body.offsetWidth, (hamburger.offsetTop + hamburger.offsetHeight) / document.body.offsetHeight)
        
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
