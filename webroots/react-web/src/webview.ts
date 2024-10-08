export declare type WebViewType = {
    showDevTools: () => void,
    request: <T, TR>(method: string, data: T) => Promise<TR>
}

