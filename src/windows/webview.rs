use std::{mem, rc::Rc};

use once_cell::unsync::OnceCell;
use webview2::{Controller, Environment, WebResourceContext};
use winapi::{shared::windef::{HWND, RECT}, um::winuser::GetClientRect};
use crate::{windows::app::App};

pub struct WebView {
    controller: Rc<OnceCell<Controller>>
}

impl WebView {
    pub fn new() -> Self {
        WebView {
            controller: Rc::new(OnceCell::<Controller>::new())
        }
    }
    pub fn initialize(&self, hwnd: HWND, url: String, enable_devtools: bool) {
        let controller_cell = self.controller.clone();
        Environment::builder().build(move | env| {
            match env {
                Ok(env) => env.create_controller(hwnd, move | controller | unsafe { 
                    let controller = controller.unwrap();
                    let mut rect = mem::zeroed();
                    GetClientRect(hwnd, &mut rect);
                    controller.clone().put_bounds(rect).unwrap();
                    let web_view = controller.clone().get_webview().unwrap();

                    let settings = web_view.get_settings().unwrap();
                    settings.put_is_script_enabled(true).unwrap();
                    settings.put_are_default_script_dialogs_enabled(true).unwrap();
                    settings.put_are_dev_tools_enabled(enable_devtools).unwrap();
                    settings.put_are_default_context_menus_enabled(false).unwrap();

                    let initial_theme = "themeWindows";
                    let script = format!("initialTheme = '{}'", initial_theme);
                    web_view.add_script_to_execute_on_document_created(&script, |_|Ok(())).unwrap();

                    web_view.add_web_resource_requested_filter("https://test/*", WebResourceContext::All).unwrap();
                    println!("Test");
                    web_view.add_web_resource_requested(|a, b | {
                        println!("Test in callback {:?} {:?}", a, b);
                        Ok(())
                    }).unwrap();

                    web_view.navigate(&url).unwrap();
                    let controller_to_borrow = controller.clone();
                    web_view.add_navigation_completed(move |_,_|{
                        controller_to_borrow.move_focus(webview2::MoveFocusReason::Next).unwrap();
                        Ok(())
                    }).unwrap();

                    controller_cell.set(controller).unwrap();
                    Ok(())
                }),
                Err(err) => {
                    App::error_message_box(&format!("Konnte WebView nicht anlegen: {}", err));
                    Err(err)
                } 
            }
        }).unwrap();
    }

    pub fn on_size(&self, rect: RECT) {
        if let Some(controller) = self.controller.get() {
            controller.put_bounds(rect).unwrap();
        }
    }

    pub fn on_move(&self) {
        if let Some(controller) = self.controller.get() {
            controller.notify_parent_window_position_changed().unwrap();
        }
    }

    pub fn set_visible(&self, visible: bool) {
        if let Some(controller) = self.controller.get() {
            controller.put_is_visible(visible).unwrap();
        }
    }

    pub fn on_focus(&self) {
        if let Some(controller) = self.controller.get() {
            let _ = controller.move_focus(webview2::MoveFocusReason::Next);
        }
    }
}

// TODO
// m_webView->AddWebResourceRequestedFilter(
//     L"*", COREWEBVIEW2_WEB_RESOURCE_CONTEXT_IMAGE);
// CHECK_FAILURE(m_webView->add_WebResourceRequested(
//     Callback<ICoreWebView2WebResourceRequestedEventHandler>(
//         [this](
//             ICoreWebView2* sender,
//             ICoreWebView2WebResourceRequestedEventArgs* args) {
//                 COREWEBVIEW2_WEB_RESOURCE_CONTEXT resourceContext;
//                 CHECK_FAILURE(args->get_ResourceContext(&resourceContext));
//                 // Ensure that the type is image
//                 if (resourceContext != COREWEBVIEW2_WEB_RESOURCE_CONTEXT_IMAGE)
//                 {
//                     return E_INVALIDARG;
//                 }
//                 // Override the response with an empty one to block the image.
//                 // If put_Response is not called, the request will continue as normal.
//                 wil::com_ptr<ICoreWebView2WebResourceResponse> response;
//                 wil::com_ptr<ICoreWebView2Environment> environment;
//                 wil::com_ptr<ICoreWebView2_2> webview2;
//                 CHECK_FAILURE(m_webView->QueryInterface(IID_PPV_ARGS(&webview2)));
//                 CHECK_FAILURE(webview2->get_Environment(&environment));
//                 CHECK_FAILURE(environment->CreateWebResourceResponse(
//                     nullptr, 403 /*NoContent*/, L"Blocked", L"Content-Type: image/jpeg",
//                     &response));
//                 CHECK_FAILURE(args->put_Response(response.get()));
//                 return S_OK;
//         })
//     .Get(),
//             &m_webResourceRequestedTokenForImageBlocking));