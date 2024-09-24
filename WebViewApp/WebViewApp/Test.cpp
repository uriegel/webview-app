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




// TODO webview2

//
//#include "Control.h"
//#include <wrl.h>
//#include "../packages/Microsoft.Windows.ImplementationLibrary.1.0.240122.1/include/wil/com.h"
//#include "../packages/Microsoft.Web.WebView2.1.0.2210.55/build/native/include/WebView2.h"
//#include "Conversion.h"
//#include <thread>
//using namespace std;
//using namespace Microsoft::WRL;
//
//struct Control : ICefControl
//{
//	void __stdcall Destroy() {
//
//	}
//	void __stdcall Resize(int width, int height) const {
//		if (webviewController) {
//			RECT bounds;
//			GetClientRect(parent, &bounds);
//			webviewController->put_Bounds(bounds);
//			ShowWindow(parent, SW_SHOW);
//			auto affe = GetWindow(parent, GW_CHILD);
//			ShowWindow(affe, SW_SHOW);
//		}
//	}
//	void __stdcall LoadUrl(const char* url) const {
//		auto wurl = url
//			| to_str
//			| to_utf16;
//		//auto waffe = wurl | wc_str;
//		auto waffel = wurl.c_str();
//		webview->Navigate(waffel);
//		//RECT bounds;
//		//GetClientRect(parent, &bounds);
//		//webviewController->put_Bounds(bounds);
//		//ShowWindow(parent, SW_SHOW);
//		//auto affe = GetWindow(parent, GW_CHILD);
//		//ShowWindow(affe, SW_SHOW);
//		//webview->Navigate(L"https://www.bing.com/");
//
//	}
//	void __stdcall ShowDevTools() const {
//
//	}
//	// Result: Unwrap() aufrufen oder mit GlobalFree freigeben!!
//	EvaluateScriptResult* __stdcall EvaluateScript(const char* script) const {
//		return nullptr;
//	}
//	void __stdcall AsyncEvaluateScript(const char* script, CallbackFunction* state, OnEvaluateScriptResultFunction* onResult) const {
//
//	}
//	bool __stdcall LoadHtmlString(const char* html, const char* url = nullptr) const {
//		return true;
//	}
//	void __stdcall SetAcceleratorTable(HANDLE acceleratorTable) const {
//
//	}
//	void __stdcall AddAccelerator(int commandId, int keyCode, bool shiftPressed, bool ctrlPressed, bool altPressed) const {
//
//	}
//	void __stdcall RemoveAccelerator(int commandId) const {
//
//	}
//	void __stdcall SetZoomLevel(int level) const {
//
//	}
//	void __stdcall ResetZoomLevel() const {
//
//	}
//	int __stdcall GetZoomLevel() const {
//		return 5;
//	}
//	void __stdcall Copy() const {
//
//	}
//	void __stdcall Cut() const {
//
//	}
//	void __stdcall Paste() const {
//
//	}
//	void __stdcall SelectAll() const {
//
//	}
//	void __stdcall LoadHtmlStringInFrame(long long frameId, const char* html, const char* url = nullptr) const {
//
//	}
//	void __stdcall Print() const {
//
//	}
//	void __stdcall Reload(bool ignoreCache = false) const {
//
//	}
//	void __stdcall GoBack() const {
//
//	}
//	void __stdcall GoForward() const {
//
//	}
//	void __stdcall Stop() const {
//
//	}
//	void __stdcall DisableMouseWheelZoom() const {
//
//	}
//	void __stdcall EnableDropFiles(bool enableDropFiles) const {
//
//	}
//	void __stdcall AsyncEvaluateUtf8Script(const char* script, CallbackFunction* state, OnEvaluateScriptResultFunction* onResult) const {
//
//	}
//	// Result: Unwrap() aufrufen oder mit GlobalFree freigeben!!
//	EvaluateScriptResult* __stdcall EvaluateUtf8Script(const char* script) const {
//		return nullptr;
//	}
//
//	HRESULT __stdcall QueryInterface(REFIID riid, void** ppvObject) {
//		return 0;
//	}
//
//
//	ULONG STDMETHODCALLTYPE AddRef(void) {
//		return ++refCount;
//	}
//
//	ULONG STDMETHODCALLTYPE Release(void) {
//		auto res = --refCount;
//		if (res == 0)
//			delete this;
//		return res;
//	}
//
//	Control(HWND parent, ICefControlCallback* callback) {
//		this->parent = parent;
//
//		auto finished = CreateEvent(nullptr, TRUE, FALSE, nullptr);
//		auto t = thread([finished]() {
//			Sleep(3000);
//			SetEvent(finished);
//			});
//		t.detach();
//		MSG msg;
//		while (WaitForSingleObject(finished, 0) != 0
//			&& GetMessage(&msg, nullptr, 0, 0))
//			DispatchMessage(&msg);
//
//
//		CreateCoreWebView2EnvironmentWithOptions(nullptr, nullptr, nullptr,
//			Callback<ICoreWebView2CreateCoreWebView2EnvironmentCompletedHandler>(
//				[parent, finished, this](HRESULT result, ICoreWebView2Environment* env) -> HRESULT {
//					// Create a CoreWebView2Controller and get the associated CoreWebView2 whose parent is the main window hWnd
//					env->CreateCoreWebView2Controller(parent, Callback<ICoreWebView2CreateCoreWebView2ControllerCompletedHandler>(
//						[parent, finished, this](HRESULT result, ICoreWebView2Controller* controller) -> HRESULT {
//							if (controller != nullptr) {
//								webviewController = controller;
//								webviewController->get_CoreWebView2(&webview);
//							}
//
//							wil::com_ptr<ICoreWebView2Settings> settings;
//							webview->get_Settings(&settings);
//							settings->put_IsScriptEnabled(TRUE);
//							settings->put_AreDefaultScriptDialogsEnabled(TRUE);
//							settings->put_IsWebMessageEnabled(TRUE);
//
//							//webview->Navigate(L"https://www.bing.com/");
//							webview->Navigate(L"about:blank");
//							SetEvent(finished);
//							return S_OK;
//						}).Get());
//					return S_OK;
//				}).Get());
//		ResetEvent(finished);
//		while (WaitForSingleObject(finished, 0) != 0
//			&& GetMessage(&msg, nullptr, 0, 0))
//			DispatchMessage(&msg);
//	}
//
//	HWND parent;
//	wil::com_ptr<ICoreWebView2> webview;
//	wil::com_ptr<ICoreWebView2Controller> webviewController;
//	int refCount{ 1 };
//};
//
//IUnknown* CreateControl(HWND parent, ICefControlCallback* callback, int version) {
//	return new Control(parent, callback);
//}


//=======================================================================

//< ? xml version = "1.0" encoding = "utf-8" ? >
//<Project DefaultTargets = "Build" xmlns = "http://schemas.microsoft.com/developer/msbuild/2003">
//<ItemGroup Label = "ProjectConfigurations">
//<ProjectConfiguration Include = "Debug|Win32">
//<Configuration>Debug< / Configuration>
//<Platform>Win32< / Platform>
//< / ProjectConfiguration>
//<ProjectConfiguration Include = "Release|Win32">
//<Configuration>Release< / Configuration>
//<Platform>Win32< / Platform>
//< / ProjectConfiguration>
//<ProjectConfiguration Include = "Debug|x64">
//<Configuration>Debug< / Configuration>
//<Platform>x64< / Platform>
//< / ProjectConfiguration>
//<ProjectConfiguration Include = "Release|x64">
//<Configuration>Release< / Configuration>
//<Platform>x64< / Platform>
//< / ProjectConfiguration>
//< / ItemGroup>
//<PropertyGroup Label = "Globals">
//< VCProjectVersion>17.0 < / VCProjectVersion >
//<Keyword>Win32Proj< / Keyword>
//<ProjectGuid>{19d01e07 - 6999 - 4c69 - 9b28 - 1e20574d5185}< / ProjectGuid>
//<RootNamespace>WebViewControl< / RootNamespace>
//< WindowsTargetPlatformVersion>10.0 < / WindowsTargetPlatformVersion >
//<SccProjectName>SAK< / SccProjectName>
//<SccAuxPath>SAK< / SccAuxPath>
//<SccLocalPath>SAK< / SccLocalPath>
//<SccProvider>SAK< / SccProvider>
//< / PropertyGroup>
//<Import Project = "$(VCTargetsPath)\Microsoft.Cpp.Default.props" / >
//<PropertyGroup Condition = "'$(Configuration)|$(Platform)'=='Debug|Win32'" Label = "Configuration">
//<ConfigurationType>DynamicLibrary< / ConfigurationType>
//<UseDebugLibraries>true< / UseDebugLibraries>
//<PlatformToolset>v143< / PlatformToolset>
//<CharacterSet>Unicode< / CharacterSet>
//< / PropertyGroup>
//<PropertyGroup Condition = "'$(Configuration)|$(Platform)'=='Release|Win32'" Label = "Configuration">
//<ConfigurationType>DynamicLibrary< / ConfigurationType>
//<UseDebugLibraries>false< / UseDebugLibraries>
//<PlatformToolset>v143< / PlatformToolset>
//<WholeProgramOptimization>true< / WholeProgramOptimization>
//<CharacterSet>Unicode< / CharacterSet>
//< / PropertyGroup>
//<PropertyGroup Condition = "'$(Configuration)|$(Platform)'=='Debug|x64'" Label = "Configuration">
//<ConfigurationType>DynamicLibrary< / ConfigurationType>
//<UseDebugLibraries>true< / UseDebugLibraries>
//<PlatformToolset>v143< / PlatformToolset>
//<CharacterSet>Unicode< / CharacterSet>
//< / PropertyGroup>
//<PropertyGroup Condition = "'$(Configuration)|$(Platform)'=='Release|x64'" Label = "Configuration">
//<ConfigurationType>DynamicLibrary< / ConfigurationType>
//<UseDebugLibraries>false< / UseDebugLibraries>
//<PlatformToolset>v143< / PlatformToolset>
//<WholeProgramOptimization>true< / WholeProgramOptimization>
//<CharacterSet>Unicode< / CharacterSet>
//< / PropertyGroup>
//<Import Project = "$(VCTargetsPath)\Microsoft.Cpp.props" / >
//<ImportGroup Label = "ExtensionSettings">
//< / ImportGroup>
//<ImportGroup Label = "Shared">
//< / ImportGroup>
//<ImportGroup Label = "PropertySheets" Condition = "'$(Configuration)|$(Platform)'=='Debug|Win32'">
//<Import Project = "$(UserRootDir)\Microsoft.Cpp.$(Platform).user.props" Condition = "exists('$(UserRootDir)\Microsoft.Cpp.$(Platform).user.props')" Label = "LocalAppDataPlatform" / >
//< / ImportGroup>
//<ImportGroup Label = "PropertySheets" Condition = "'$(Configuration)|$(Platform)'=='Release|Win32'">
//<Import Project = "$(UserRootDir)\Microsoft.Cpp.$(Platform).user.props" Condition = "exists('$(UserRootDir)\Microsoft.Cpp.$(Platform).user.props')" Label = "LocalAppDataPlatform" / >
//< / ImportGroup>
//<ImportGroup Label = "PropertySheets" Condition = "'$(Configuration)|$(Platform)'=='Debug|x64'">
//<Import Project = "$(UserRootDir)\Microsoft.Cpp.$(Platform).user.props" Condition = "exists('$(UserRootDir)\Microsoft.Cpp.$(Platform).user.props')" Label = "LocalAppDataPlatform" / >
//< / ImportGroup>
//<ImportGroup Label = "PropertySheets" Condition = "'$(Configuration)|$(Platform)'=='Release|x64'">
//<Import Project = "$(UserRootDir)\Microsoft.Cpp.$(Platform).user.props" Condition = "exists('$(UserRootDir)\Microsoft.Cpp.$(Platform).user.props')" Label = "LocalAppDataPlatform" / >
//< / ImportGroup>
//<PropertyGroup Label = "UserMacros" / >
//<ItemDefinitionGroup Condition = "'$(Configuration)|$(Platform)'=='Debug|Win32'">
//<ClCompile>
//<WarningLevel>Level3< / WarningLevel>
//<SDLCheck>true< / SDLCheck>
//<PreprocessorDefinitions>WIN32; _DEBUG; WEBVIEWCONTROL_EXPORTS; _WINDOWS; _USRDLL; % (PreprocessorDefinitions) < / PreprocessorDefinitions >
//<ConformanceMode>true< / ConformanceMode>
//<LanguageStandard>stdcpplatest< / LanguageStandard>
//<EnableModules>true< / EnableModules>
//<BuildStlModules>true< / BuildStlModules>
//< / ClCompile>
//<Link>
//<SubSystem>Windows< / SubSystem>
//<GenerateDebugInformation>true< / GenerateDebugInformation>
//<EnableUAC>false< / EnableUAC>
//<ModuleDefinitionFile>Source.def< / ModuleDefinitionFile>
//< / Link>
//< / ItemDefinitionGroup>
//<ItemDefinitionGroup Condition = "'$(Configuration)|$(Platform)'=='Release|Win32'">
//<ClCompile>
//<WarningLevel>Level3< / WarningLevel>
//<FunctionLevelLinking>true< / FunctionLevelLinking>
//<IntrinsicFunctions>true< / IntrinsicFunctions>
//<SDLCheck>true< / SDLCheck>
//<PreprocessorDefinitions>WIN32; NDEBUG; WEBVIEWCONTROL_EXPORTS; _WINDOWS; _USRDLL; % (PreprocessorDefinitions) < / PreprocessorDefinitions >
//<ConformanceMode>true< / ConformanceMode>
//<LanguageStandard>stdcpplatest< / LanguageStandard>
//<EnableModules>true< / EnableModules>
//<BuildStlModules>true< / BuildStlModules>
//< / ClCompile>
//<Link>
//<SubSystem>Windows< / SubSystem>
//<EnableCOMDATFolding>true< / EnableCOMDATFolding>
//<OptimizeReferences>true< / OptimizeReferences>
//<GenerateDebugInformation>true< / GenerateDebugInformation>
//<EnableUAC>false< / EnableUAC>
//<ModuleDefinitionFile>Source.def< / ModuleDefinitionFile>
//< / Link>
//< / ItemDefinitionGroup>
//<ItemDefinitionGroup Condition = "'$(Configuration)|$(Platform)'=='Debug|x64'">
//<ClCompile>
//<WarningLevel>Level3< / WarningLevel>
//<SDLCheck>true< / SDLCheck>
//<PreprocessorDefinitions>_DEBUG; WEBVIEWCONTROL_EXPORTS; _WINDOWS; _USRDLL; % (PreprocessorDefinitions) < / PreprocessorDefinitions >
//<ConformanceMode>true< / ConformanceMode>
//<LanguageStandard>stdcpplatest< / LanguageStandard>
//<EnableModules>true< / EnableModules>
//<BuildStlModules>true< / BuildStlModules>
//< / ClCompile>
//<Link>
//<SubSystem>Windows< / SubSystem>
//<GenerateDebugInformation>true< / GenerateDebugInformation>
//<EnableUAC>false< / EnableUAC>
//<ModuleDefinitionFile>Source.def< / ModuleDefinitionFile>
//< / Link>
//< / ItemDefinitionGroup>
//<ItemDefinitionGroup Condition = "'$(Configuration)|$(Platform)'=='Release|x64'">
//<ClCompile>
//<WarningLevel>Level3< / WarningLevel>
//<FunctionLevelLinking>true< / FunctionLevelLinking>
//<IntrinsicFunctions>true< / IntrinsicFunctions>
//<SDLCheck>true< / SDLCheck>
//<PreprocessorDefinitions>NDEBUG; WEBVIEWCONTROL_EXPORTS; _WINDOWS; _USRDLL; % (PreprocessorDefinitions) < / PreprocessorDefinitions >
//<ConformanceMode>true< / ConformanceMode>
//<LanguageStandard>stdcpplatest< / LanguageStandard>
//<EnableModules>true< / EnableModules>
//<BuildStlModules>true< / BuildStlModules>
//< / ClCompile>
//<Link>
//<SubSystem>Windows< / SubSystem>
//<EnableCOMDATFolding>true< / EnableCOMDATFolding>
//<OptimizeReferences>true< / OptimizeReferences>
//<GenerateDebugInformation>true< / GenerateDebugInformation>
//<EnableUAC>false< / EnableUAC>
//<ModuleDefinitionFile>Source.def< / ModuleDefinitionFile>
//< / Link>
//< / ItemDefinitionGroup>
//<ItemGroup>
//<ClInclude Include = "Control.h" / >
//<ClInclude Include = "Conversion.h" / >
//<ClInclude Include = "framework.h" / >
//< / ItemGroup>
//<ItemGroup>
//<ClCompile Include = "Control.cpp" / >
//<ClCompile Include = "Conversion.cpp" / >
//<ClCompile Include = "dllmain.cpp" / >
//<ClCompile Include = "Loader.cpp" / >
//< / ItemGroup>
//<ItemGroup>
//<None Include = "packages.config" / >
//<None Include = "Source.def" / >
//< / ItemGroup>
//<Import Project = "$(VCTargetsPath)\Microsoft.Cpp.targets" / >
//<ImportGroup Label = "ExtensionTargets">
//<Import Project = "..\packages\Microsoft.Windows.ImplementationLibrary.1.0.240122.1\build\native\Microsoft.Windows.ImplementationLibrary.targets" Condition = "Exists('..\packages\Microsoft.Windows.ImplementationLibrary.1.0.240122.1\build\native\Microsoft.Windows.ImplementationLibrary.targets')" / >
//<Import Project = "..\packages\Microsoft.Web.WebView2.1.0.2210.55\build\native\Microsoft.Web.WebView2.targets" Condition = "Exists('..\packages\Microsoft.Web.WebView2.1.0.2210.55\build\native\Microsoft.Web.WebView2.targets')" / >
//< / ImportGroup>
//<Target Name = "EnsureNuGetPackageBuildImports" BeforeTargets = "PrepareForBuild">
//<PropertyGroup>
//<ErrorText>This project references NuGet package(s) that are missing on this computer.Use NuGet Package Restore to download them.For more information, see http ://go.microsoft.com/fwlink/?LinkID=322105. The missing file is {0}.</ErrorText>
//< / PropertyGroup>
//<Error Condition = "!Exists('..\packages\Microsoft.Windows.ImplementationLibrary.1.0.240122.1\build\native\Microsoft.Windows.ImplementationLibrary.targets')" Text = "$([System.String]::Format('$(ErrorText)', '..\packages\Microsoft.Windows.ImplementationLibrary.1.0.240122.1\build\native\Microsoft.Windows.ImplementationLibrary.targets'))" / >
//<Error Condition = "!Exists('..\packages\Microsoft.Web.WebView2.1.0.2210.55\build\native\Microsoft.Web.WebView2.targets')" Text = "$([System.String]::Format('$(ErrorText)', '..\packages\Microsoft.Web.WebView2.1.0.2210.55\build\native\Microsoft.Web.WebView2.targets'))" / >
//< / Target>
//< / Project>