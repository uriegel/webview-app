use std::{ffi::c_void, ptr};

use windows::Win32::Foundation::{DRAGDROP_S_CANCEL, DRAGDROP_S_DROP, DRAGDROP_S_USEDEFAULTCURSORS, E_NOINTERFACE, S_OK};
use windows_core::{GUID, HRESULT};

use super::{IID_IDROPSOURCE, IID_IUNKNOWN};

#[repr(C)]
pub struct IDropSource {
    vtbl: *const IDropSourceVTable,
    ref_count: u32
}

#[repr(C)]
struct IDropSourceVTable {
    query_interface: unsafe extern "system" fn(this: *mut c_void, riid: *const GUID, ppv: *mut *mut c_void)->HRESULT,
    add_ref: unsafe extern "system" fn(this: *mut c_void)->u32,
    release: unsafe extern "system" fn(this: *mut c_void)->u32,
    query_continue_drag: unsafe extern "system" fn(this: *mut c_void, escape_pressed: i32, key_state: i32)->HRESULT,
    give_feedback: unsafe extern "system" fn(this: *mut c_void, effect: i32)->HRESULT
}

static DROP_SOURCE_VTABLE: IDropSourceVTable = IDropSourceVTable {
    query_interface: IDropSource::query_interface,
    add_ref: IDropSource::add_ref,
    release: IDropSource::release,
    query_continue_drag: IDropSource::query_continue_drag,
    give_feedback: IDropSource::give_feedback
};

impl IDropSource {
    pub fn new()->*mut IDropSource {
        let drop_source = Box::new(IDropSource {
            vtbl: &DROP_SOURCE_VTABLE,
            ref_count: 1
        });   
        Box::into_raw(drop_source)
    }

    unsafe extern "system" fn query_interface(this: *mut c_void, riid: *const GUID, ppv: *mut *mut c_void) -> HRESULT {
        let self_ptr = this as *mut IDropSource;
        if *riid == IID_IUNKNOWN || *riid == IID_IDROPSOURCE {
            ((*self_ptr).vtbl.as_ref().unwrap().add_ref)(this);
            *ppv = self_ptr as *mut _;
            S_OK
        } else {
            *ppv = ptr::null_mut();
            E_NOINTERFACE
        }
    }

    unsafe extern "system" fn add_ref(this: *mut c_void) -> u32 {
        let self_ptr = this as *mut IDropSource;
        (*self_ptr).ref_count += 1;
        (*self_ptr).ref_count
    }

    unsafe extern "system" fn release(this: *mut c_void) -> u32 {
        let self_ptr = this as *mut IDropSource;
        (*self_ptr).ref_count -= 1;
        if (*self_ptr).ref_count == 0 {
            let _ = Box::from_raw(self_ptr);
        }
        (*self_ptr).ref_count
    }

    unsafe extern "system" fn query_continue_drag(_this: *mut c_void, escape_pressed: i32, key_state: i32)->HRESULT {
        const MK_LBUTTON: i32 = 1;
        
        if escape_pressed != 0 {
            DRAGDROP_S_CANCEL
        } else if (key_state & MK_LBUTTON) == 0 {
            DRAGDROP_S_DROP
        } else {
            S_OK
        }
    }
    
    unsafe extern "system" fn give_feedback(_this: *mut c_void, _effect: i32)->HRESULT {
        DRAGDROP_S_USEDEFAULTCURSORS
    }
}