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

using OnCloseFunc = bool(void* target, int x, int y, int w, int h, bool isMaximized);
using OnCustomRequestFunc = void(void* target, const wchar_t* url, int urlLen, RequestResult* requestResult);
wil::com_ptr<ICoreWebView2> webview;
wil::com_ptr<ICoreWebView2Controller> webviewController;

struct WebViewAppSettings {
    const wchar_t* title;
    const wchar_t* userDataPath;
    int x;
    int y;
    int width;
    int height;
    bool isMaximized;
    void* target;
    OnCloseFunc* OnClose;
    OnCustomRequestFunc* OnCustomRequest;
    const wchar_t* url;
    bool withoutNativeTitlebar;
    bool customResourceScheme;
    bool devtools;
    bool defaultContextmenu;
};

wchar_t* title { nullptr };
wchar_t* userDataPath { nullptr };
int x;
int y;
int width;
int height;
bool isMaximized;
void* target;
OnCloseFunc* OnClose;
OnCustomRequestFunc* OnCustomRequest;
wchar_t* url { nullptr };
auto withoutNativeTitlebar = false;
auto customResourceScheme = false;
bool devtools;
bool defaultContextmenu;

wchar_t* SetString(const wchar_t* str) {
    auto len = wcslen(str) + 1;
    auto target = new wchar_t[len];
    wcscpy_s(target, len, str);
    return target;
}

void Init(const WebViewAppSettings* settings) {
    auto hr = CoInitialize(nullptr);
    DpiUtil::SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);

    title = SetString(settings->title);
    url = SetString(settings->url);
    x = settings->x;
    y = settings->y;
    width = settings->width;
    height = settings->height;
    isMaximized = settings->isMaximized;
    target = settings->target;
    OnClose = settings->OnClose;
    OnCustomRequest = settings->OnCustomRequest;
    userDataPath = SetString(settings->userDataPath);
    withoutNativeTitlebar = settings->withoutNativeTitlebar;
    customResourceScheme = settings->customResourceScheme;
    devtools = settings->devtools;
    defaultContextmenu = settings->defaultContextmenu;
}

void CreateWebView(HWND hWnd) {
    auto options = Microsoft::WRL::Make<CoreWebView2EnvironmentOptions>();
    auto scheme = Microsoft::WRL::Make<CoreWebView2CustomSchemeRegistration>(L"res");
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
                            webview->AddWebResourceRequestedFilter(L"res:*", COREWEBVIEW2_WEB_RESOURCE_CONTEXT_ALL);
                            webview->add_WebResourceRequested(
                                Callback<ICoreWebView2WebResourceRequestedEventHandler>(
                                    [env](ICoreWebView2* sender, ICoreWebView2WebResourceRequestedEventArgs* args) {
                                        wil::com_ptr<ICoreWebView2WebResourceRequest> request;
                                        args->get_Request(&request);
                                        wchar_t* uri;
                                        request->get_Uri(&uri);
                                        RequestResult rr{ 0 };
                                        OnCustomRequest(target, uri, (int)wcslen(uri), &rr);
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
                                            auto text = R"(!DOCTYPE html>
<html>
<head>
    <title>Not Found</title>
    <meta charset="utf-8">
</head>
<body>
    <h1>Not Found</h1>
                    
    <p>
        Sorry, I cannot find what you're looking for
    </p>
</body>
</html>)";
                                            auto stream = SHCreateMemStream((const BYTE*)text, (int)strlen(text));
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
                        ShowWindow(webviewWnd, SW_SHOW);
                        webview->Navigate(url);
                        delete[] url;
                        url = nullptr;
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
            }
        break;
        case WM_CLOSE:
            {
                RECT rect;
                GetWindowRect(hWnd, &rect);
                if (OnClose(target, rect.left, rect.top, rect.right-rect.left, rect.bottom - rect.top, IsZoomed(hWnd)))
                    DestroyWindow(hWnd);
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

wchar_t* __stdcall Test1(wchar_t* text_to_display) {
    MessageBoxW(NULL, text_to_display, L"C�ptschn", MB_OK);
    auto txt = L"Das ist ein sch�ner Result";
    auto len = wcslen(txt);
    auto text = new wchar_t[len + 1];
    wcscpy_s(text, len + 1, txt);
    return text;
}

void Free(wchar_t* txt_ptr) {
    delete[] txt_ptr;
}
