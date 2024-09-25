#include "pch.h"
#include "objbase.h"
#include <wrl.h>
#include "../packages/Microsoft.Windows.ImplementationLibrary.1.0.240803.1/include/wil/com.h"
#include "../packages/Microsoft.Web.WebView2.1.0.2792.45/build/native/include/WebView2.h"
#include "../packages/Microsoft.Web.WebView2.1.0.2792.45/build/native/include/WebView2EnvironmentOptions.h"
using namespace Microsoft::WRL;

auto WINDOW_CLASS = L"$$WebView_APP$$";

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
    const wchar_t* url;
    bool withoutNativeTitlebar;
};

wchar_t* title { nullptr };
wchar_t* userDataPath { nullptr };
int x;
int y;
int width;
int height;
bool isMaximized;
wchar_t* url { nullptr };
auto withoutNativeTitlebar = false;

wchar_t* SetString(const wchar_t* str) {
    auto len = wcslen(str) + 1;
    auto target = new wchar_t[len];
    wcscpy_s(target, len, str);
    return target;
}


void Init(const WebViewAppSettings* settings) {
    auto hr = CoInitialize(nullptr);
    title = SetString(settings->title);
    url = SetString(settings->url);
    x = settings->x;
    y = settings->y;
    width = settings->width;
    height = settings->height;
    isMaximized = settings->isMaximized;
    userDataPath = SetString(settings->userDataPath);
    withoutNativeTitlebar = settings->withoutNativeTitlebar;
}

void CreateWebView(HWND hWnd) {
    auto options = Microsoft::WRL::Make<CoreWebView2EnvironmentOptions>();
    if (withoutNativeTitlebar)
        options->put_AdditionalBrowserArguments(L"--enable-features=msWebView2EnableDraggableRegions");
    CreateCoreWebView2EnvironmentWithOptions(nullptr, userDataPath, options.Get(),
        Callback<ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler>(
            [hWnd](HRESULT result, ICoreWebView2Environment* env) -> HRESULT {
                // Create a CoreWebView2Controller and get the associated CoreWebView2 whose parent is the main window hWnd
                env->CreateCoreWebView2Controller(hWnd, Callback<ICoreWebView2CreateCoreWebView2ControllerCompletedHandler>(
                    [hWnd](HRESULT result, ICoreWebView2Controller* controller) -> HRESULT {
                        if (controller != nullptr) {
                            webviewController = controller;
                            webviewController->get_CoreWebView2(&webview);
                        }

                        wil::com_ptr<ICoreWebView2Settings> settings;
                        webview->get_Settings(&settings);
                        settings->put_IsScriptEnabled(TRUE);
                        settings->put_AreDefaultScriptDialogsEnabled(TRUE);
                        settings->put_IsWebMessageEnabled(TRUE);

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
    // TODO ICON: build.rs: make resource file with icon 1, version 
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

size_t Strlen(const wchar_t* txt_ptr) {
    return wcslen(txt_ptr);
}

void Free(wchar_t* txt_ptr) {
    delete[] txt_ptr;
}
