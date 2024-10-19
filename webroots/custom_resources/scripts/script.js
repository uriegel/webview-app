console.log("script loaded")
const btn1 = document.getElementById("button")
const btn2 = document.getElementById("button2")
const btn3 = document.getElementById("button3")
const btnE = document.getElementById("buttonE")
const btnDevTools = document.getElementById("buttonDevTools")
const dragzone = document.getElementById("dragzone")

btnDevTools.onclick = () => WebView.showDevTools()

const initialize = async () => {
    WebView.setDroppedFilesEventHandler(success => console.log("Files dropped", success))
}
try {
    if (WebView)
        initialize()
} catch {  }

function onWebViewLoaded() {
    initialize()
}

function onEvent(id) {
    console.log("Event received", id)
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

btnE.onclick = async () => await WebView.request("cmdE", {})

btn3.onclick = () => {
    const video = document.getElementById("video")
    video.classList.remove("hidden")
    video.src="http://roxy:8080/media/video/2010.mp4"
}

dragzone.onmousedown = () => WebView.startDragFiles([
        "TestApp.dll",
        "FSharpTools.dll"
    ]
    .map(n => `${currentDirectory}${n}`)
)
