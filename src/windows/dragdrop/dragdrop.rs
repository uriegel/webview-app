use std::ptr::null;

use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use windows_core::{PCSTR, PCWSTR};

use crate::windows::string_to_pcwstr;

use super::{dataobject::IDataObject, dropsource::IDropSource};

type OleInitialize = unsafe extern "stdcall" fn(*const core::ffi::c_void) -> i32;
type DoDragDrop = unsafe extern "stdcall" fn(*mut IDataObject, *mut IDropSource, i32, *mut i32) -> i32;

pub fn start(files: Vec<String>) {
    unsafe {
        let ole2 = string_to_pcwstr("Ole32.dll");
        let lib =  LoadLibraryW(PCWSTR(ole2.as_ptr())).unwrap();
        let ole_init: OleInitialize = match GetProcAddress(lib, PCSTR("OleInitialize\0".as_ptr())) {
            Some(addr) => std::mem::transmute(addr),
            None => panic!("Failed to get address of OleInitialize"),
        };
        let _ = ole_init(null());

        let start_drag: DoDragDrop = match GetProcAddress(lib, PCSTR("DoDragDrop\0".as_ptr())) {
            Some(addr) => std::mem::transmute(addr),
            None => panic!("Failed to get address of DoDragDrop"),
        };

        let dataobject = IDataObject::new(files);
        let dropsource = IDropSource::new();

        let mut effect = 0;
        let _res = start_drag(dataobject, dropsource, 1|2, &mut effect);
    }    
}