use std::{cell::RefCell, collections::HashMap, mem, path::Path, ptr, rc::Rc, sync::mpsc};

use serde::Deserialize;
use serde_json::Value;
use webview2_com::{
    AddScriptToExecuteOnDocumentCreatedCompletedHandler, CoTaskMemPWSTR, CoreWebView2CustomSchemeRegistration, CoreWebView2EnvironmentOptions, 
    CreateCoreWebView2ControllerCompletedHandler, CreateCoreWebView2EnvironmentCompletedHandler, ExecuteScriptCompletedHandler, 
    Microsoft::Web::WebView2::Win32::{
        CreateCoreWebView2EnvironmentWithOptions, ICoreWebView2, ICoreWebView2Controller, ICoreWebView2CustomSchemeRegistration, 
        ICoreWebView2Environment, ICoreWebView2EnvironmentOptions, ICoreWebView2Settings6, ICoreWebView2WebResourceResponse, 
        COREWEBVIEW2_WEB_RESOURCE_CONTEXT_ALL
    }, NavigationCompletedEventHandler, WebMessageReceivedEventHandler, WebResourceRequestedEventHandler
};

use windows::Win32::{
    Foundation::{
        E_POINTER, HWND, LPARAM, RECT, SIZE, WPARAM
    }, Graphics::Gdi::UpdateWindow, System::{
        Com::IStream, Threading, WinRT::EventRegistrationToken
    }, UI::{
        Input::KeyboardAndMouse, WindowsAndMessaging::{
            DispatchMessageW, GetClientRect, GetMessageW, PostQuitMessage, PostThreadMessageW, ShowWindow, TranslateMessage, GWLP_USERDATA, 
            MSG, SW_SHOW, WM_APP 
        }
    }
};
use windows_sys::Win32::UI::Shell::SHCreateMemStream;
use windows_core::{w, Interface, PWSTR};

use crate::{bounds::Bounds, content_type, html, javascript, params::Params, request::Request};

use super::{framewindow::FrameWindow, GetWindowLong, SetWindowLong};

struct WebViewController(ICoreWebView2Controller);

#[derive(Debug)]
enum Error {
    Error
}

type Result<T> = std::result::Result<T, Error>;

type WebViewSender = mpsc::Sender<Box<dyn FnOnce(WebView) + Send>>;
type WebViewReceiver = mpsc::Receiver<Box<dyn FnOnce(WebView) + Send>>;
type BindingCallback = Box<dyn FnMut(Vec<Value>) -> Result<Value>>;
type BindingsMap = HashMap<String, BindingCallback>;

#[derive(Clone)]
pub struct WebView {
    controller: Rc<WebViewController>,
    webview: Rc<ICoreWebView2>,
    tx: WebViewSender,
    rx: Rc<WebViewReceiver>,
    thread_id: u32,
    pub frame: FrameWindow,
    should_save_bounds: bool,
    config_dir: String,
    can_close: Rc<RefCell<Box<dyn Fn()->bool + 'static>>>
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
                    let user_data_path = CoTaskMemPWSTR::from(local_path_clone.as_os_str().to_str().unwrap());
                    CreateCoreWebView2EnvironmentWithOptions(None, *user_data_path.as_ref().as_pcwstr(), &options,  &environmentcreatedhandler) // TODO with options
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
            should_save_bounds: params.save_bounds,
            config_dir: local_path.to_string_lossy().to_string(),
            can_close: Rc::new(RefCell::new(Box::new(||true))),
        };

        webview
            .init(&javascript::get(params.without_native_titlebar, &title, true, false))
            .unwrap();

        unsafe {
            let mut _token = EventRegistrationToken::default();
            webview.webview.add_WebMessageReceived(
                &WebMessageReceivedEventHandler::create(Box::new(move |_webview, args| {
                    if let Some(args) = args {
                        let mut message = PWSTR(ptr::null_mut());
                        if args.TryGetWebMessageAsString(&mut message).is_ok() {
                            let message = CoTaskMemPWSTR::from(message);
                            let msg = &message.to_string();
                            if params.devtools && msg == "devtools" {
                                _webview.unwrap().OpenDevToolsWindow().unwrap();
                            }
            
                            
  //                          if let Ok(value) =
//                                serde_json::from_str::<InvokeMessage>(&message.to_string())
                            {
                                // let mut bindings = bindings.borrow_mut();
                                // if let Some(f) = bindings.get_mut(&value.method) {
                                //     match (*f)(value.params) {
                                //         Ok(result) => bound.resolve(value.id, 0, result),
                                //         Err(err) => bound.resolve(
                                //             value.id,
                                //             1,
                                //             Value::String(format!("{err:#?}")),
                                //         ),
                                //     }
                                //     .unwrap();
                                // }
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
                let url = CoTaskMemPWSTR::from(url.as_str());
                println!("Test: {}", url);


                //webview.webview.AddWebResourceRequestedFilter(*url.as_ref().as_pcwstr(), COREWEBVIEW2_WEB_RESOURCE_CONTEXT_ALL).unwrap();
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

        let url = CoTaskMemPWSTR::from(url.as_str());
        unsafe { 
            webview.webview.Navigate(*url.as_ref().as_pcwstr()).unwrap() 
        };
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

    fn init(&self, js: &str) -> Result<&Self> {
        let webview = self.webview.clone();
        let js = String::from(js);
        AddScriptToExecuteOnDocumentCreatedCompletedHandler::wait_for_async_operation(
            Box::new(move |handler| unsafe {
                let js = CoTaskMemPWSTR::from(js.as_str());
                webview
                    .AddScriptToExecuteOnDocumentCreated(*js.as_ref().as_pcwstr(), &handler)
                    .map_err(webview2_com::Error::WindowsError)
            }),
            Box::new(|error_code, _id| error_code),
        ).unwrap();
        Ok(self)
    }

    fn resolve(&self, id: u64, status: i32, result: Value) -> Result<&Self> {
        let result = result.to_string();

        self.dispatch(move |webview| {
            let method = match status {
                0 => "resolve",
                _ => "reject",
            };
            let js = format!(
                r#"
                window._rpc[{id}].{method}({result});
                window._rpc[{id}] = undefined;"#
            );

            webview.eval(&js).expect("eval return script");
        })
    }

    fn eval(&self, js: &str) -> Result<&Self> {
        let webview = self.webview.clone();
        let js = String::from(js);
        ExecuteScriptCompletedHandler::wait_for_async_operation(
            Box::new(move |handler| unsafe {
                let js = CoTaskMemPWSTR::from(js.as_str());
                webview
                    .ExecuteScript(*js.as_ref().as_pcwstr(), &handler)
                    .map_err(webview2_com::Error::WindowsError)
            }),
            Box::new(|error_code, _result| error_code),
        ).unwrap();
        Ok(self)
    }

    pub fn set_size(&self, x: i32, y: i32) {
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
        let content_type = CoTaskMemPWSTR::from(content_type.as_str());
        environment.CreateWebResourceResponse(
            &stream,
            200, // HTTP Status 200 OK
            w!("OK"),
            *content_type.as_ref().as_pcwstr()
        ).unwrap()
    }
}

