use std::{ffi::c_void, ptr};

use windows::Win32::Foundation::{E_INVALIDARG, E_NOINTERFACE, S_FALSE, S_OK};
use windows_core::{GUID, HRESULT};

use super::{IID_IDROPSOURCE, IID_IUNKNOWN};

#[repr(C)]
pub struct IEnumFormatEtc {
    vtbl: *const IEnumFormatEtcVTable,
    ref_count: u32,
    formats: *const FormatEtc,
    num_formats: i32,
    current_index: i32
}

#[repr(C)]
struct IEnumFormatEtcVTable {
    query_interface: unsafe extern "system" fn(this: *mut c_void, riid: *const GUID, ppv: *mut *mut c_void)->HRESULT,
    add_ref: unsafe extern "system" fn(this: *mut c_void)->u32,
    release: unsafe extern "system" fn(this: *mut c_void)->u32,
    next: unsafe extern "system" fn(this: *mut c_void, celt: u32, rgelt: *mut FormatEtc, pcelt_fetched: *mut i32)->HRESULT,
    skip: unsafe extern "system" fn(this: *mut c_void, celt: u32)->HRESULT,
    reset: unsafe extern "system" fn(this: *mut c_void)->HRESULT,
    clone: unsafe extern "system" fn(this: *mut c_void, ppenum: *mut *mut IEnumFormatEtc)->HRESULT
}

#[repr(C)]
pub struct FormatEtc {
    pub cf_format: u16,
    ptd: *mut TargetDevice,
    dw_aspect: i32,
    lindex: i32,
    tymed: i32,
}

unsafe impl Sync for FormatEtc {}

#[repr(C)]
struct TargetDevice {
    size: u32,
    driver_name_offset: u16,
    device_name_offset: u16,
    port_name_offset: u16,
    ext_devmode_offset: u16,
    data: [u8; 1]   
}

static ENUM_FORMAT_ETC_VTABLE: IEnumFormatEtcVTable = IEnumFormatEtcVTable {
    query_interface: IEnumFormatEtc::query_interface,
    add_ref: IEnumFormatEtc::add_ref,
    release: IEnumFormatEtc::release,
    next: IEnumFormatEtc::next,
    skip: IEnumFormatEtc::skip,
    reset: IEnumFormatEtc::reset,
    clone: IEnumFormatEtc::clone,
};

static G_FORMATS: [FormatEtc; 1] = [
    FormatEtc {
        cf_format: 15,
        ptd: ptr::null_mut(),
        dw_aspect: 1,
        lindex: -1,
        tymed: 1,
    },
];

impl IEnumFormatEtc {
    pub fn new()->*mut IEnumFormatEtc {
        let format_etc = Box::new(IEnumFormatEtc {
            vtbl: &ENUM_FORMAT_ETC_VTABLE,
            ref_count: 1,
            formats: G_FORMATS.as_ptr(),
            current_index: 0,
            num_formats: 1
        });   
        Box::into_raw(format_etc)
    }

    unsafe extern "system" fn query_interface(this: *mut c_void, riid: *const GUID, ppv: *mut *mut c_void) -> HRESULT {
        let self_ptr = this as *mut IEnumFormatEtc;
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
        let self_ptr = this as *mut IEnumFormatEtc;
        (*self_ptr).ref_count += 1;
        (*self_ptr).ref_count
    }

    unsafe extern "system" fn release(this: *mut c_void) -> u32 {
        let self_ptr = this as *mut IEnumFormatEtc;
        (*self_ptr).ref_count -= 1;
        if (*self_ptr).ref_count == 0 {
            let _ = Box::from_raw(self_ptr);
        }
        (*self_ptr).ref_count
    }

    unsafe extern "system" fn next(this: *mut c_void, celt: u32, rgelt: *mut FormatEtc, pcelt_fetched: *mut i32)->HRESULT {
        let self_ptr = this as *mut IEnumFormatEtc;
        if rgelt.is_null() {
            return E_INVALIDARG;
        }

        let mut fetched = 0;
        while (*self_ptr).current_index < (*self_ptr).num_formats && fetched < celt {
            let source = (*self_ptr).formats.add((*self_ptr).current_index as usize);
            let destination = rgelt.add(fetched as usize);

            ptr::write(destination, ptr::read(source));

            (*self_ptr).current_index += 1;
            fetched += 1;
        }
        
        if !pcelt_fetched.is_null() {
            *pcelt_fetched = fetched as i32;
        }
        
        if fetched == celt {
            S_OK 
        } else { 
            S_FALSE 
        }
    }
    unsafe extern "system" fn skip(this: *mut c_void, celt: u32)->HRESULT {
        let self_ptr = this as *mut IEnumFormatEtc;
        (*self_ptr).current_index += celt as i32;
        if (*self_ptr).current_index > (*self_ptr).num_formats {
            (*self_ptr).current_index = (*self_ptr).num_formats;
            return S_FALSE
        }        
        else {
            S_OK
        }
    }
    unsafe extern "system" fn reset(this: *mut c_void)->HRESULT {
        let self_ptr = this as *mut IEnumFormatEtc;
        (*self_ptr).current_index = 0;
        S_OK

    }
    unsafe extern "system" fn clone(this: *mut c_void, ppenum: *mut *mut IEnumFormatEtc)->HRESULT {
        let self_ptr = this as *mut IEnumFormatEtc;
        if self_ptr.is_null() {
            return E_INVALIDARG;
        }

        // Allocate memory for the clone.
        let clone = Box::into_raw(Box::new(IEnumFormatEtc {
            vtbl: (*self_ptr).vtbl,
            formats: (*self_ptr).formats,
            num_formats: (*self_ptr).num_formats,
            current_index: (*self_ptr).current_index,
            ref_count: 1,
        }));

        // Assign the cloned object to the output pointer.
        *ppenum = &mut (*clone) as *mut IEnumFormatEtc;

        S_OK
    }

}


