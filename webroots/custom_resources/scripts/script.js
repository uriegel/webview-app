console.log("script loaded")
const btn1 = document.getElementById("button")
const btn2 = document.getElementById("button2")
const btnDevTools = document.getElementById("buttonDevTools")
const dragzone = document.getElementById("dragzone")

btnDevTools.onclick = () => WebView.showDevTools()

let currentDirectory = ""

const initialize = async () => {
    WebView.registerEvents("fast", console.log)
    WebView.registerEvents("slow", console.log)
    WebView.setDroppedFilesEventHandler(success => console.log("Files dropped", success))
//    currentDirectory = (await WebView.request("getCurrentDir", {})).directory
}
try {
    if (WebView)
        initialize()
} catch {  }

function onWebViewLoaded() {
    initialize()
}

btn1.onclick = async () => {
    const t0 = performance.now()

    var res = await WebView.request("cmd1", {
        text: "Text",
        id: 123
    })
    const t1 = performance.now()
    const tdiff = t1 - t0
    console.log(`${tdiff}`, "cmd1", res)
}

btn2.onclick = async () => {
    const t0 = performance.now()
    var res = await WebView.request("cmd2", { })
    const t1 = performance.now()
    const tdiff = t1 - t0
    console.log(`${tdiff}`, "cmd2", res)
}

dragzone.onmousedown = () => WebView.startDragFiles([
        "TestApp.dll",
        "FSharpTools.dll"
    ]
    .map(n => `${currentDirectory}${n}`)
)
