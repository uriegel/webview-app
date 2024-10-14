#include "pch.h"
#include "objbase.h"
#include "shlwapi.h"
#include <wrl.h>
#include "../packages/Microsoft.Windows.ImplementationLibrary.1.0.240803.1/include/wil/com.h"
#include "../packages/Microsoft.Web.WebView2.1.0.2792.45/build/native/include/WebView2.h"
#include "../packages/Microsoft.Web.WebView2.1.0.2792.45/build/native/include/WebView2EnvironmentOptions.h"
#include "DpiUtil.h"
using namespace Microsoft::WRL;

auto WINDOW_CLASS = L"$$WebView_APP$$";

struct RequestResult {
    char* content;
    size_t len;
    int status;
    wchar_t content_type[100];
};

void __stdcall PostMessageAsString(wchar_t* txt);
bool __stdcall ExecuteScript(wchar_t* script);

const auto WM_SENDSCRIPT = WM_APP + 1;

enum ShowWndType {
    ShowWndType_Restore,
    ShowWndType_Maximize,
    ShowWndType_Minimize
};

using OnCloseFunc = bool(int x, int y, int w, int h, bool isMaximized);
using OnMaximizeFunc = void(bool isMaximized);
using OnCustomRequestFunc = void(const wchar_t* url, int urlLen, RequestResult* requestResult);
using OnMessageFunc = void(const wchar_t* msg, int msgLen);
wil::com_ptr<ICoreWebView2> webview;
wil::com_ptr<ICoreWebView2Controller> webviewController;

struct WebViewAppSettings {
    const wchar_t* title;
    const wchar_t* userDataPath;
    const wchar_t* htmlOk;
    const wchar_t* htmlNotFound;
    const wchar_t* initScript;
    int x;
    int y;
    int width;
    int height;
    bool isMaximized;
    OnCloseFunc* OnClose;
    OnCustomRequestFunc* OnCustomRequest;
    OnMessageFunc* OnMessage;
    OnMaximizeFunc* OnMaximize;
    const wchar_t* url;
    bool withoutNativeTitlebar;
    bool customResourceScheme;
    bool devtools;
    bool defaultContextmenu;
};

wchar_t* title { nullptr };
wchar_t* userDataPath { nullptr };
wchar_t* htmlOk;
wchar_t* htmlNotFound;
wchar_t* initScript;
int x;
int y;
int width;
int height;
bool isMaximized;
OnCloseFunc* OnClose;
OnCustomRequestFunc* OnCustomRequest;
OnMessageFunc* OnMessage;
OnMaximizeFunc* OnMaximize;
wchar_t* url { nullptr };
auto withoutNativeTitlebar = false;
auto customResourceScheme = false;
bool devtools;
bool defaultContextmenu;
HWND hWndWebView;
bool windowIsMaximized = false;

wchar_t* SetString(const wchar_t* str) {
    auto len = wcslen(str) + 1;
    auto target = new wchar_t[len];
    wcscpy_s(target, len, str);
    return target;
}

void __stdcall Init(const WebViewAppSettings* settings) {
    auto hr = CoInitialize(nullptr);
    DpiUtil::SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);

    title = SetString(settings->title);
    url = SetString(settings->url);
    htmlOk = SetString(settings->htmlOk);
    htmlNotFound = SetString(settings->htmlNotFound);
    initScript = SetString(settings->initScript);
    x = settings->x;
    y = settings->y;
    width = settings->width;
    height = settings->height;
    isMaximized = settings->isMaximized;
    OnClose = settings->OnClose;
    OnMaximize = settings->OnMaximize;
    OnCustomRequest = settings->OnCustomRequest;
    OnMessage = settings->OnMessage;
    userDataPath = SetString(settings->userDataPath);
    withoutNativeTitlebar = settings->withoutNativeTitlebar;
    customResourceScheme = settings->customResourceScheme;
    devtools = settings->devtools;
    defaultContextmenu = settings->defaultContextmenu;
}

void CreateWebView(HWND hWnd) {
    hWndWebView = hWnd;
    auto options = Microsoft::WRL::Make<CoreWebView2EnvironmentOptions>();
    auto scheme = Microsoft::WRL::Make<CoreWebView2CustomSchemeRegistration>(L"req");
    if (withoutNativeTitlebar)
        options->put_AdditionalBrowserArguments(L"--enable-features=msWebView2EnableDraggableRegions");
    ICoreWebView2CustomSchemeRegistration* registrations[1] = { scheme.Get() };
    options->SetCustomSchemeRegistrations(1, static_cast<ICoreWebView2CustomSchemeRegistration**>(registrations));
    CreateCoreWebView2EnvironmentWithOptions(nullptr, userDataPath, options.Get(),
        Callback<ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler>(
            [hWnd](HRESULT result, ICoreWebView2Environment* env) -> HRESULT {
                // Create a CoreWebView2Controller and get the associated CoreWebView2 whose parent is the main window hWnd
                env->CreateCoreWebView2Controller(hWnd, Callback<ICoreWebView2CreateCoreWebView2ControllerCompletedHandler>(
                    [hWnd, env](HRESULT result, ICoreWebView2Controller* controller) -> HRESULT {
                        if (controller != nullptr) {
                            webviewController = controller;
                            webviewController->get_CoreWebView2(&webview);
                        }

                        wil::com_ptr<ICoreWebView2Settings> settings2;
                        webview->get_Settings(&settings2);
                        wil::com_ptr<ICoreWebView2Settings6> settings;
                        settings2->QueryInterface(&settings);
                        settings->put_IsScriptEnabled(TRUE);
                        settings->put_AreDefaultScriptDialogsEnabled(TRUE);
                        settings->put_AreBrowserAcceleratorKeysEnabled(FALSE);
                        settings->put_IsPasswordAutosaveEnabled(TRUE);
                        settings->put_IsWebMessageEnabled(TRUE);
                        settings->put_AreDefaultContextMenusEnabled(defaultContextmenu);

                        if (customResourceScheme || withoutNativeTitlebar) {
                            webview->AddWebResourceRequestedFilter(L"req:*", COREWEBVIEW2_WEB_RESOURCE_CONTEXT_ALL);
                            webview->add_WebResourceRequested(
                                Callback<ICoreWebView2WebResourceRequestedEventHandler>(
                                    [env](ICoreWebView2* sender, ICoreWebView2WebResourceRequestedEventArgs* args) {
                                        wil::com_ptr<ICoreWebView2WebResourceRequest> request;
                                        args->get_Request(&request);
                                        wchar_t* uri;
                                        request->get_Uri(&uri);

                                        if (wcsncmp(uri, L"req://webroot", 13) == 0) {
                                            RequestResult rr{ 0 };
                                            OnCustomRequest(uri, (int)wcslen(uri), &rr);
                                            CoTaskMemFree(uri);
                                            if (rr.status == 200) {
                                                auto stream = SHCreateMemStream((const BYTE*)rr.content, (int)rr.len);
                                                wil::com_ptr<ICoreWebView2WebResourceResponse> response;
                                                wchar_t ct[100];
                                                wsprintfW(ct, L"Content-Type: %s", rr.content_type);
                                                env->CreateWebResourceResponse(stream, 200, L"Ok", ct, &response);
                                                stream->Release();
                                                args->put_Response(response.get());
                                            }
                                            else if (rr.status == 404) {
                                                auto stream = SHCreateMemStream((const BYTE*)htmlNotFound, (int)wcslen(htmlNotFound) * 2);
                                                wil::com_ptr<ICoreWebView2WebResourceResponse> response;
                                                env->CreateWebResourceResponse(stream, 404, L"Not Found", L"Content-Type: text/html", &response);
                                                stream->Release();
                                                args->put_Response(response.get());
                                            }
                                        }
                                        else {
                                            auto stream = SHCreateMemStream((const BYTE*)htmlNotFound, (int)wcslen(htmlNotFound) * 2);
                                            wil::com_ptr<ICoreWebView2WebResourceResponse> response;
                                            env->CreateWebResourceResponse(stream, 404, L"Not Found", L"Content-Type: text/html", &response);
                                            stream->Release();
                                            args->put_Response(response.get());
                                        }

                                        return S_OK;
                                    }).Get(), nullptr);
                        }
                        webview->add_WindowCloseRequested(
                            Callback<ICoreWebView2WindowCloseRequestedEventHandler>(
                                [hWnd](ICoreWebView2* _, IUnknown* args) -> HRESULT {
                                    CloseWindow(hWnd);
                                    return S_OK;
                                }).Get(), nullptr);
                        RECT bounds;
                        GetClientRect(hWnd, &bounds);
                        webviewController->put_Bounds(bounds);
                        ShowWindow(hWnd, SW_SHOW);
                        auto webviewWnd = GetWindow(hWnd, GW_CHILD);
                        webview->add_WebMessageReceived(
                            Callback<ICoreWebView2WebMessageReceivedEventHandler>(
                                [](ICoreWebView2* sender, ICoreWebView2WebMessageReceivedEventArgs* args) {
                                    wil::unique_cotaskmem_string messageRaw;
                                    wil::unique_cotaskmem_string message;
                                    args->TryGetWebMessageAsString(&message);
                                    OnMessage(message.get(), (int)wcslen(message.get()));
                                    return S_OK;
                                }).Get(), nullptr);

                        webview->Navigate(url);
                        delete[] url;
                        url = nullptr;
                        ExecuteScript(initScript);
                        delete[] initScript;
                        initScript = nullptr;
                        ShowWindow(webviewWnd, SW_SHOW);
                        return S_OK;
                    }).Get());
                return S_OK;
            }).Get());
}

LRESULT CALLBACK WndProc(HWND hWnd, UINT message, WPARAM wParam, LPARAM lParam) {
    switch (message) {
        case WM_CREATE:
            CreateWebView(hWnd);
            if (isMaximized)
                ShowWindow(hWnd, SW_MAXIMIZE);
            break;
        case WM_NCCALCSIZE:
            if (withoutNativeTitlebar) {
                auto isZoomed = IsZoomed(hWnd);
                auto isZoomedTop = isZoomed ? 7 : 0;
                auto isZoomedAll = isZoomed ? 3 : 0;
                
                if (wParam == TRUE) {
                    auto params = (NCCALCSIZE_PARAMS*)lParam;
                    params->rgrc[0].top += 1 + isZoomedTop;
                    params->rgrc[0].bottom -= 5 + isZoomedAll;
                    params->rgrc[0].left += 5 + isZoomedAll;
                    params->rgrc[0].right -= 5 + isZoomedAll;
                }
                else {
                    auto params = (RECT*)lParam;
                    params->top += 1 + isZoomedTop;
                    params->bottom -= 5 + isZoomedAll;
                    params->left += 5 + isZoomedAll;
                    params->right -= 5 + isZoomedAll;
                }
                return 0;
            }
            else
                return DefWindowProc(hWnd, message, wParam, lParam);
            break;
        case WM_SIZE:
            if (webviewController) {
                RECT bounds;
                GetClientRect(hWnd, &bounds);
                webviewController->put_Bounds(bounds);
                if (wParam == SIZE_MAXIMIZED && !windowIsMaximized) {
                    OnMaximize(true);
                    windowIsMaximized = true;
                }
                if (wParam == SIZE_RESTORED && windowIsMaximized) {
                    OnMaximize(false);
                    windowIsMaximized = false;
                }
            }
        break;
        case WM_CLOSE:
            {
                RECT rect;
                GetWindowRect(hWnd, &rect);
                if (OnClose(rect.left, rect.top, rect.right-rect.left, rect.bottom - rect.top, IsZoomed(hWnd)))
                    DestroyWindow(hWnd);
            }
            break;
        case WM_SENDSCRIPT:
            {
                auto script = (wchar_t*)lParam;
                PostMessageAsString(script);
                delete [] script;
            }
            break;
        case WM_DESTROY:
            PostQuitMessage(0);
            break;
        default:
            return DefWindowProc(hWnd, message, wParam, lParam);
        }
    return 0;
}

auto RegisterClass(HINSTANCE hInstance) {
    WNDCLASSEXW wcex{ 0 };
    wcex.cbSize = sizeof(WNDCLASSEX);
    wcex.style = CS_HREDRAW | CS_VREDRAW;
    wcex.lpfnWndProc = WndProc;
    wcex.cbClsExtra = 0;
    wcex.cbWndExtra = 0;
    wcex.hInstance = hInstance;
    //wcex.hIcon = LoadIcon(hInstance, MAKEINTRESOURCE(1));
    wcex.hCursor = LoadCursor(nullptr, IDC_ARROW);
    wcex.hbrBackground = (HBRUSH)(COLOR_WINDOW + 1);
    wcex.lpszClassName = WINDOW_CLASS;
    return RegisterClassExW(&wcex);
}

BOOL InitInstance(HINSTANCE hInstance, int nCmdShow)
{
    HWND hWnd = CreateWindowW(WINDOW_CLASS, title, WS_OVERLAPPEDWINDOW,
        x == -1 ? CW_USEDEFAULT: x, y == -1 ? CW_USEDEFAULT : y, 
        width == -1 ? CW_USEDEFAULT : width, height == -1 ? CW_USEDEFAULT : height,
        nullptr, nullptr, hInstance, nullptr);
    delete[] title;
    title = nullptr;
    if (!hWnd)
        return FALSE;

    ShowWindow(hWnd, nCmdShow);
    UpdateWindow(hWnd);

    return TRUE;
}

int __stdcall Run() {
    auto instance = GetModuleHandle(nullptr);
    RegisterClass(instance);

    // Anwendungsinitialisierung ausf�hren:
    if (!InitInstance(instance, SW_NORMAL))
        return FALSE;

    MSG msg;
    while (GetMessage(&msg, nullptr, 0, 0))
    {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }

    return (int)msg.wParam;
}

bool __stdcall ExecuteScript(wchar_t* script) {
    webview->ExecuteScript(script,
        Callback<ICoreWebView2ExecuteScriptCompletedHandler>(
            [](HRESULT error, LPCWSTR result)->HRESULT {
                return S_OK;
            }).Get());
    return true;
}

void __stdcall PostMessageAsString(wchar_t* txt) {
    webview->PostWebMessageAsString(txt);
}

void __stdcall PostMessageAsJson(wchar_t* json) {
    webview->PostWebMessageAsJson(json);
}

void __stdcall ShowDevTools() {
    webview->OpenDevToolsWindow();
}

wchar_t* __stdcall Test1(wchar_t* text_to_display) {
    MessageBoxW(NULL, text_to_display, L"C�ptschn", MB_OK);
    auto txt = L"Das ist ein sch�ner Result";
    auto len = wcslen(txt);
    auto text = new wchar_t[len + 1];
    wcscpy_s(text, len + 1, txt);
    return text;
}

void __stdcall SendText(char* text, int len) {
    auto wlen = MultiByteToWideChar(CP_UTF8, 0, text, len, nullptr, 0);
    if (len > 1) {
        auto ret = new wchar_t[wlen + 1];
        if (MultiByteToWideChar(CP_UTF8, 0, text, len, ret, wlen) > 1) {
            ret[wlen] = 0;
            PostMessage(hWndWebView, WM_SENDSCRIPT, 0, (LPARAM)ret);
        }
        else
            delete [] ret;
    }
    else
        PostMessage(hWndWebView, WM_SENDSCRIPT, 0, 0);
}

void __stdcall Free(wchar_t* txt_ptr) {
    delete[] txt_ptr;
}

int matchShowWindowType(ShowWndType type) {
    switch (type) {
    case ShowWndType_Minimize:
        return 6;
    case ShowWndType_Maximize:
        return 3;
    default:
        return 9;
    }
}

void __stdcall ShowWnd(ShowWndType type) {
    ShowWindow(hWndWebView, matchShowWindowType(type));
}

void __stdcall CloseWnd() {
    SendMessage(hWndWebView, WM_CLOSE, 0, 0);
}