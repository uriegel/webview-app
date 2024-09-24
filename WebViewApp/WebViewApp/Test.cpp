#include "pch.h"

auto WINDOW_CLASS = L"$$WebView_APP$$";

struct WebViewAppSettings {
    const wchar_t* title;
};

wchar_t* title;

void Init(const WebViewAppSettings* settings) {
    auto len = wcslen(settings->title) + 1;
    title = new wchar_t[len];
    wcscpy_s(title, len, settings->title);
}

LRESULT CALLBACK WndProc(HWND hWnd, UINT message, WPARAM wParam, LPARAM lParam) {
    switch (message) {
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
    // TODO ICON
    //wcex.hIcon = LoadIcon(hInstance, MAKEINTRESOURCE(IDI_TESTER));
    //wcex.hIconSm = LoadIcon(wcex.hInstance, MAKEINTRESOURCE(IDI_SMALL));
    wcex.hCursor = LoadCursor(nullptr, IDC_ARROW);
    wcex.hbrBackground = (HBRUSH)(COLOR_WINDOW + 1);
    wcex.lpszClassName = WINDOW_CLASS;
    return RegisterClassExW(&wcex);
}

BOOL InitInstance(HINSTANCE hInstance, int nCmdShow)
{
    HWND hWnd = CreateWindowW(WINDOW_CLASS, title, WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT, 0, CW_USEDEFAULT, 0, nullptr, nullptr, hInstance, nullptr);
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

    // Anwendungsinitialisierung ausführen:
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
    MessageBoxW(NULL, text_to_display, L"Cäptschn", MB_OK);
    auto txt = L"Das ist ein schöner Result";
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
