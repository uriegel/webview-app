use std::{cell::RefCell, ffi::c_void, mem, path::Path, ptr, rc::Rc, sync::mpsc};

use webview2_com::{
    AddScriptToExecuteOnDocumentCreatedCompletedHandler, CoTaskMemPWSTR, CoreWebView2CustomSchemeRegistration, CoreWebView2EnvironmentOptions, 
    CreateCoreWebView2ControllerCompletedHandler, CreateCoreWebView2EnvironmentCompletedHandler, ExecuteScriptCompletedHandler, 
    Microsoft::Web::WebView2::Win32::{
        CreateCoreWebView2EnvironmentWithOptions, ICoreWebView2, ICoreWebView2Controller, ICoreWebView2CustomSchemeRegistration, ICoreWebView2Environment, 
        ICoreWebView2EnvironmentOptions, ICoreWebView2Settings6, ICoreWebView2WebResourceResponse, COREWEBVIEW2_WEB_RESOURCE_CONTEXT_ALL
    }, NavigationCompletedEventHandler, WebMessageReceivedEventHandler, WebResourceRequestedEventHandler, WindowCloseRequestedEventHandler
};

use windows::Win32::{
    Foundation::{
        E_POINTER, HWND, LPARAM, RECT, SIZE, WPARAM
    }, Graphics::Gdi::UpdateWindow, System::{
        Com::{CoTaskMemFree, IStream}, Threading, WinRT::EventRegistrationToken
    }, UI::{
        Input::KeyboardAndMouse, WindowsAndMessaging::{
            DispatchMessageW, GetClientRect, GetMessageW, PostMessageW, PostQuitMessage, PostThreadMessageW, SendMessageW, SetWindowPos, ShowWindow, TranslateMessage, GWLP_USERDATA, HWND_TOP, MSG, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE, SW_SHOW, SW_SHOWMAXIMIZED, SW_SHOWMINIMIZED, SW_SHOWNORMAL, WM_APP, WM_CLOSE 
        }
    }
};
use windows_sys::Win32::UI::Shell::SHCreateMemStream;
use windows_core::{w, Interface, PCWSTR, PWSTR};

use crate::{bounds::Bounds, content_type, html, javascript::{self, RequestData}, params::Params, request::Request};

use super::{framewindow::FrameWindow, string_to_pcwstr, GetWindowLong, SetWindowLong};

pub const WM_SENDRESPONSE: u32 = WM_APP + 1;
pub const WM_SENDSCRIPT: u32 = WM_APP + 2;

struct WebViewController(ICoreWebView2Controller);

#[derive(Debug)]
pub enum Error {
    Error
}

type Result<T> = std::result::Result<T, Error>;

type WebViewSender = mpsc::Sender<Box<dyn FnOnce(WebView) + Send>>;
type WebViewReceiver = mpsc::Receiver<Box<dyn FnOnce(WebView) + Send>>;

#[derive(Clone)]
pub struct WebView {
    pub frame: FrameWindow,
    pub without_native_titlebar: bool,
    controller: Rc<WebViewController>,
    webview: Rc<ICoreWebView2>,
    tx: WebViewSender,
    rx: Rc<WebViewReceiver>,
    thread_id: u32,
    should_save_bounds: bool,
    config_dir: String,
    can_close: Rc<RefCell<Box<dyn Fn()->bool + 'static>>>,
    on_request: Rc<RefCell<Box<dyn Fn(&Request, String, String, String) -> bool + 'static>>>,
    is_maximized: Rc<RefCell<bool>>
}


impl Drop for WebViewController {
    fn drop(&mut self) {
        unsafe { self.0.Close() }.unwrap();
    }
}

impl WebView {
    pub fn new(params: Params)->WebView {
        let app_data = std::env::var("LOCALAPPDATA").expect("No APP_DATA directory");
        let local_path = Path::new(&app_data).join(params.app.get_appid());
        let bounds = 
            if params.save_bounds
                { Bounds::restore(&local_path.to_string_lossy()).unwrap_or(params.bounds) } 
            else
                { params.bounds};
        let title = params.title.unwrap_or_else(||"Webview App".to_string());
        let frame = FrameWindow::new(&title.clone(), bounds);
        let parent = *frame.window;

        let environment = {
            let (tx, rx) = mpsc::channel();

            let options = CoreWebView2EnvironmentOptions::default();
            if params.without_native_titlebar {
                unsafe { options.set_additional_browser_arguments("--enable-features=msWebView2EnableDraggableRegions".to_string()) };
            }
            let scheme_registration = CoreWebView2CustomSchemeRegistration::new("req".to_string());
            unsafe { options.set_scheme_registrations(vec![Some(ICoreWebView2CustomSchemeRegistration::from(scheme_registration))]); }

            let local_path_clone = local_path.clone();
            CreateCoreWebView2EnvironmentCompletedHandler::wait_for_async_operation(
                Box::new(move |environmentcreatedhandler| unsafe {
                    let options: ICoreWebView2EnvironmentOptions = ICoreWebView2EnvironmentOptions::from(options);
                    let user_data_path = string_to_pcwstr(local_path_clone.as_os_str().to_str().unwrap());
                    CreateCoreWebView2EnvironmentWithOptions(None, PCWSTR(user_data_path.as_ptr()), &options, 
                            &environmentcreatedhandler) 
                        .map_err(webview2_com::Error::WindowsError)
                }),
                Box::new(move |error_code, environment| {
                    error_code?;
                    tx.send(environment.ok_or_else(|| windows::core::Error::from(E_POINTER)))
                        .expect("send over mpsc channel");
                    Ok(())
                }),
            ).unwrap();

            rx.recv()
                .map_err(|_| Error::Error).unwrap()
        }.unwrap();

        let environment_clone = environment.clone();
        let controller = {
            let (tx, rx) = mpsc::channel();

            CreateCoreWebView2ControllerCompletedHandler::wait_for_async_operation(
                Box::new(move |handler| unsafe {
                    environment
                        .CreateCoreWebView2Controller(parent, &handler)
                        .map_err(webview2_com::Error::WindowsError)
                }),
                Box::new(move |error_code, controller| {
                    error_code?;
                    tx.send(controller.ok_or_else(|| windows::core::Error::from(E_POINTER)))
                        .expect("send over mpsc channel");
                    Ok(())
                }),
            ).unwrap();

            rx.recv()
                .map_err(|_| Error::Error).unwrap()
        }.unwrap();

        let size = get_window_size(parent);
        let mut client_rect = RECT::default();
        unsafe {
            let _ = GetClientRect(parent, &mut client_rect);
            controller.SetBounds(RECT {
                left: 0,
                top: 0,
                right: size.cx,
                bottom: size.cy,
            }).unwrap();
            controller.SetIsVisible(true).unwrap();
        }

        let webview = unsafe { controller.CoreWebView2().unwrap() };

        unsafe {
            let settings = webview.Settings().unwrap();

            settings.SetAreDefaultContextMenusEnabled(params.default_contextmenu).unwrap();
            settings.SetAreDevToolsEnabled(params.devtools).unwrap();

            settings.SetIsScriptEnabled(true).unwrap();
            settings.SetAreDefaultScriptDialogsEnabled(    true).unwrap();
            settings.SetIsWebMessageEnabled(true).unwrap();
            let settings6: ICoreWebView2Settings6 = settings.cast().unwrap();
            settings6.SetAreBrowserAcceleratorKeysEnabled(false).unwrap();
            settings6.SetIsPasswordAutosaveEnabled(true).unwrap();
        }

        *frame.size.borrow_mut() = size;

        let (tx, rx) = mpsc::channel();
        let rx = Rc::new(rx);
        let thread_id = unsafe { Threading::GetCurrentThreadId() };

        let with_webroot = params.webroot.is_some();
        let (url, custom_resource_scheme) = match (params.url, params.debug_url, with_webroot) {
            (None, None, true) => ("req://webroot/index.html".to_string(), true),
            (Some(url), None, _) => (url, true),
            (_, Some(debug_url), _) => (debug_url, false),
            (_, _, _) => ("about:plain".to_string(), false)
        };

        let webview = WebView {
            controller: Rc::new(WebViewController(controller)),
            webview: Rc::new(webview),
            tx,
            rx,
            thread_id,
            frame,
            without_native_titlebar: params.without_native_titlebar,
            should_save_bounds: params.save_bounds,
            config_dir: local_path.to_string_lossy().to_string(),
            can_close: Rc::new(RefCell::new(Box::new(||true))),
            on_request: Rc::new(RefCell::new(Box::new(|_,_,_,_|false))),
            is_maximized: Rc::new(RefCell::new(false))
        };

        webview
            .init(&javascript::get(params.without_native_titlebar, &title, true, false))
            .unwrap();

        unsafe {
            let mut _token = EventRegistrationToken::default();
            let hwnd = webview.frame.get_hwnd();
            webview.webview.add_WindowCloseRequested(
                &WindowCloseRequestedEventHandler::create(Box::new(move|_,_| {
                    let hwnd = hwnd as *mut c_void;
                    let hwnd = HWND(hwnd);
                    SendMessageW(hwnd, WM_CLOSE, WPARAM(0), LPARAM(0));
                    Ok(())
                })),
                &mut _token,
            ).unwrap();

            let mut _token = EventRegistrationToken::default();
            let webview_clone = webview.clone();
            let hwnd = webview.frame.get_hwnd();
            webview.webview.add_WebMessageReceived(
                &WebMessageReceivedEventHandler::create(Box::new(move |_webview, args| {
                    if let Some(args) = args {
                        let mut message = PWSTR(ptr::null_mut());
                        if args.TryGetWebMessageAsString(&mut message).is_ok() {
                            let message = CoTaskMemPWSTR::from(message);
                            let msg = &message.to_string();
                            if params.devtools && msg == "devtools" {
                                _webview.unwrap().OpenDevToolsWindow().unwrap();
                            } else if msg.starts_with("request,") {
                                let request_data = RequestData::new(&msg);
                                let on_request = webview_clone.on_request.borrow();
                                let request = Request { hwnd };
                                on_request(&request, request_data.id.to_string(), request_data.cmd.to_string(), request_data.json.to_string());
                            } else if msg.starts_with("MaximizeWindow") {
                                let hwnd = hwnd as *mut c_void;
                                let hwnd = HWND(hwnd);
                                ShowWindow(hwnd, SW_SHOWMAXIMIZED).unwrap();
                            } else if msg.starts_with("MinimizeWindow") {
                                let hwnd = hwnd as *mut c_void;
                                let hwnd = HWND(hwnd);
                                ShowWindow(hwnd, SW_SHOWMINIMIZED).unwrap();
                            } else if msg.starts_with("RestoreWindow") {
                                let hwnd = hwnd as *mut c_void;
                                let hwnd = HWND(hwnd);
                                ShowWindow(hwnd, SW_SHOWNORMAL).unwrap();
                            }
                        }
                    }
                    Ok(())
                })),
                &mut _token,
            ).unwrap();
        }

        if custom_resource_scheme || params.without_native_titlebar {
            unsafe {
                webview.webview.AddWebResourceRequestedFilter(w!("req:*"), COREWEBVIEW2_WEB_RESOURCE_CONTEXT_ALL).unwrap();
                let mut _token = EventRegistrationToken::default();

                webview.webview.add_WebResourceRequested(
                    &WebResourceRequestedEventHandler::create(Box::new(move |_, args| {
                        if let Some(args) = args {
                            let request = args.Request().unwrap();
                            let mut uri = PWSTR(ptr::null_mut());
                            request.Uri(&mut uri).unwrap();
                            let uri = CoTaskMemPWSTR::from(uri);
                            let mut uri = uri.to_string();
                            if uri.starts_with("req://webroot") {
                                let path = uri.split_off(14);
                                match params.webroot.clone().expect("Custom request without webroot").lock().unwrap().get_file(path.clone()) {
                                    Some(file)  => {
                                        let content = file.contents();
                                        let response = send_custom_response(&environment_clone, content, &path);
                                        args.SetResponse(&response).unwrap();
                                    },
                                    None => {
                                        let response = send_custom_response(&environment_clone, html::not_found().as_bytes(), ".html");
                                        args.SetResponse(&response).unwrap();
                                    } 
                                }
                            }
                            Ok(())
                        } else {
                            Ok(())
                        }
                    })), &mut _token).unwrap();
            }
        }

        let url = string_to_pcwstr(url.as_str());
        unsafe { 
            webview.webview.Navigate(PCWSTR(url.as_ptr())).unwrap(); 
        }
        WebView::set_window_webview(parent, Some(Box::new(webview.clone())));
        webview
    }

    pub fn can_close(&self, val: impl Fn()->bool + 'static) {
        let _ = self.can_close.replace(Box::new(val));
    }

    pub fn connect_request<F: Fn(&Request, String, String, String) -> bool + 'static>(
        &self,
        on_request: F,
    ) {
        let _ = self.on_request.replace(Box::new(on_request));
    }

    pub fn run(self) {
        let webview = self.webview.as_ref();
        let (tx, rx) = mpsc::channel();

        let handler =
            NavigationCompletedEventHandler::create(Box::new(move |_sender, _args| {
                tx.send(()).expect("send over mpsc channel");
                Ok(())
            }));
        let mut token = EventRegistrationToken::default();
        unsafe {
            webview.add_NavigationCompleted(&handler, &mut token).unwrap();
            let result = webview2_com::wait_with_pump(rx);
            webview.remove_NavigationCompleted(token).unwrap();
            result.unwrap();
        }

        unsafe {
            let _ = ShowWindow(*self.frame.window, SW_SHOW);
            let _ = UpdateWindow(*self.frame.window);
            let _ = SetWindowPos(*self.frame.window, HWND_TOP, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED);
            let _ = KeyboardAndMouse::SetFocus(*self.frame.window);
        }

        let mut msg = MSG::default();
        let h_wnd = HWND::default();

        loop {
            while let Ok(f) = self.rx.try_recv() {
                (f)(self.clone());
            }

            unsafe {
                let result = GetMessageW(&mut msg, h_wnd, 0, 0).0;

                match result {
                    -1 => break, // Err(windows::core::Error::from_win32().into()),
                    0 => break, // Ok(()),
                    _ => match msg.message {
                        WM_APP => (),
                        _ => {
                            let _ = TranslateMessage(&msg);
                            DispatchMessageW(&msg);
                        }
                    },
                }
            }
        }
    }

    pub fn terminate(self) {
        self.dispatch(|_webview| unsafe {
            PostQuitMessage(0);
        }).unwrap();
    }

    pub fn send_response(&self, response: WPARAM) {
        let ptr: PWSTR = PWSTR(response.0 as *mut u16);
        unsafe { 
            self.webview.PostWebMessageAsString(ptr).unwrap();
            CoTaskMemFree(Some(ptr.0 as *mut _));
        }
    }

    fn init(&self, js: &str) -> Result<&Self> {
        let webview = self.webview.clone();
        let js = String::from(js);
        AddScriptToExecuteOnDocumentCreatedCompletedHandler::wait_for_async_operation(
            Box::new(move |handler| unsafe {
                let js = string_to_pcwstr(js.as_str());
                webview
                    .AddScriptToExecuteOnDocumentCreated(PCWSTR(js.as_ptr()), &handler)
                    .map_err(webview2_com::Error::WindowsError)
            }),
            Box::new(|error_code, _id| error_code),
        ).unwrap();
        Ok(self)
    }

    pub fn evaluate_script(&self, js: &str) {
        let js = string_to_pcwstr(js);
        unsafe { self.webview
            .ExecuteScript(PCWSTR(js.as_ptr()), &ExecuteScriptCompletedHandler::create(Box::new(|k, _|k))).unwrap() };
    }

    pub fn eval(&self, js: &str) -> Result<&Self> {
        let webview = self.webview.clone();
        let js = String::from(js);
        ExecuteScriptCompletedHandler::wait_for_async_operation(
            Box::new(move |handler| unsafe {
                let js = string_to_pcwstr(js.as_str());
                webview
                    .ExecuteScript(PCWSTR(js.as_ptr()), &handler)
                    .map_err(webview2_com::Error::WindowsError)
            }),
            Box::new(|error_code, _result| error_code),
        ).unwrap();
        Ok(self)
    }

    pub fn set_size(&self, x: i32, y: i32, is_maximized: bool) {
        unsafe {
            self.controller
                .0
                .SetBounds(RECT {
                    left: 0,
                    top: 0,
                    right: x,
                    bottom: y,
                }).unwrap();
        };
        if is_maximized != *self.is_maximized.borrow() {
            self.is_maximized.replace(is_maximized);

            let js: String = format!("WEBVIEWsetMaximized({is_maximized})");
            let mut js = CoTaskMemPWSTR::from(js.as_str());
            let wparam: WPARAM = WPARAM(js.take().as_ptr() as usize);
            let lparam: LPARAM = LPARAM(0);   
            let hwnd = self.frame.get_hwnd();
            let hwnd = hwnd as *mut c_void;
            let hwnd = HWND(hwnd);
            unsafe { PostMessageW(hwnd, WM_SENDSCRIPT, wparam, lparam).unwrap() };
        }
    }

    pub fn on_close(&self, x: i32, y: i32, w: i32, h: i32, is_maximized: bool)->bool {
        let can_close_fn = self.can_close.borrow();
        let can_close = can_close_fn();
        if can_close && self.should_save_bounds {
            let bounds = Bounds {
                x: Some(x),
                y: Some(y),
                width: Some(w),
                height: Some(h),
                is_maximized 
            };
            bounds.save(&self.config_dir);
        }
        can_close
    }

    fn set_window_webview(hwnd: HWND, webview: Option<Box<WebView>>) -> Option<Box<WebView>> {
        unsafe {
            match SetWindowLong(
                hwnd,
                GWLP_USERDATA,
                match webview {
                    Some(webview) => Box::into_raw(webview) as _,
                    None => 0_isize,
                },
            ) {
                0 => None,
                ptr => Some(Box::from_raw(ptr as *mut _)),
            }
        }
    }

    pub fn get_window_webview(hwnd: HWND) -> Option<Box<WebView>> {
        unsafe {
            let data = GetWindowLong(hwnd, GWLP_USERDATA);

            match data {
                0 => None,
                _ => {
                    let webview_ptr = data as *mut WebView;
                    let raw = Box::from_raw(webview_ptr);
                    let webview = raw.clone();
                    mem::forget(raw);

                    Some(webview)
                }
            }
        }
    }

    fn dispatch<F>(&self, f: F) -> Result<&Self>
    where
        F: FnOnce(WebView) + Send + 'static,
    {
        self.tx.send(Box::new(f)).expect("send the fn");

        unsafe {
            let _ = PostThreadMessageW(
                self.thread_id,
                WM_APP,
                WPARAM::default(),
                LPARAM::default(),
            );
        }
        Ok(self)
    }
}

fn get_window_size(hwnd: HWND) -> SIZE {
    let mut client_rect = RECT::default();
    let _ = unsafe { GetClientRect(hwnd, &mut client_rect) };
    SIZE {
        cx: client_rect.right - client_rect.left,
        cy: client_rect.bottom - client_rect.top,
    }
}

fn send_custom_response(environment: &ICoreWebView2Environment, content: &[u8], url: &str)-> ICoreWebView2WebResourceResponse {
    unsafe {
        let stream = SHCreateMemStream(content.as_ptr(), content.len() as u32);
        let stream = IStream::from_raw(stream);    

        let content_type = format!("Content-Type: {}", content_type::get(url));
        let content_type = string_to_pcwstr(content_type.as_str());
        environment.CreateWebResourceResponse(
            &stream,
            200, // HTTP Status 200 OK
            w!("OK"),
            PCWSTR(content_type.as_ptr())
        ).unwrap()
    }
}

