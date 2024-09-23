#include "pch.h"

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
