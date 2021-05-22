use std::{mem, rc::Rc};

use once_cell::unsync::OnceCell;
use webview2::{Controller, Environment};
use winapi::{shared::windef::{HWND, RECT}, um::winuser::GetClientRect};
use crate::{app::AppSettings, windows::app::App};

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
            controller.move_focus(webview2::MoveFocusReason::Next).unwrap();
        }
    }
}