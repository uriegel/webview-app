import './App.css'
import "./webview.ts"
import { WebViewType } from './webview.ts'

declare var WebView: WebViewType

type Input = {
    text: string,
    id: number
}

type Contact = {
    name: string,
    id: number
}

function App() {

	const onDevTools = () => WebView.showDevTools()
	const onCmd = async () => {
		let res = await WebView.request<Input, Contact>("cmd1", {
			text: "Text",
			id: 123
		})
		console.log("result", res)
	}

	return (
		<>
			<h1>React test</h1>
			<p>
				
				<button onClick={onDevTools}>Dev Tools</button>
				<button onClick={onCmd}>Cmd</button>
			</p>
		</>
  	)
}

export default App
