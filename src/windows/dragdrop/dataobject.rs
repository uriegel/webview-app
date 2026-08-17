use std::{ffi::c_void, mem, ptr::{self, null}};

use windows::Win32::{Foundation::{E_FAIL, E_NOINTERFACE, E_NOTIMPL, S_OK}, System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GHND}};
use windows_core::{GUID, HRESULT};

use crate::windows::string_to_pcwstr;

use super::{enumformatetc::{FormatEtc, IEnumFormatEtc}, IID_IDATAOBJECT, IID_IUNKNOWN};

#[repr(C)]
pub struct IDataObject {
    vtbl: *const IDataObjectVTable,
    ref_count: u32,
    files: Vec<String>
}

#[repr(C)]
struct IUnknown {
    query_interface: unsafe extern "system" fn(this: *mut c_void, riid: *const GUID, ppv: *mut *mut c_void)->HRESULT,
    add_ref: unsafe extern "system" fn(this: *mut c_void)->u32,
    release: unsafe extern "system" fn(this: *mut c_void)->u32,
}

#[repr(C)]
struct StgMedium {
    tymed: i32,
    global: *mut c_void,
    punk_for_release: *const IUnknown
}

#[repr(C)]
struct DropFiles {
    files: i32,
    pt: Point, 
    nc: i32,   
    wide: i32
}

#[repr(C)]
struct Point {
    x: i32,
    y: i32
}

struct IAdviseSink {}
struct IEnumSTATDATA {}

#[repr(C)]
struct IDataObjectVTable {
    query_interface: unsafe extern "system" fn(this: *mut c_void, riid: *const GUID, ppv: *mut *mut c_void)->HRESULT,
    add_ref: unsafe extern "system" fn(this: *mut c_void)->u32,
    release: unsafe extern "system" fn(this: *mut c_void)->u32,
    get_data: unsafe extern "system" fn(this: *mut c_void, format_etc: *mut FormatEtc, medium: *mut StgMedium)->HRESULT,
    get_data_here: unsafe extern "system" fn(this: *mut c_void, medium: *mut StgMedium)->HRESULT,
    get_canonical_formatetc: unsafe extern "system" fn(this: *mut c_void, format_etc_in: *mut FormatEtc, format_etc_out: *mut FormatEtc)->HRESULT,
    set_data: unsafe extern "system" fn(this: *mut c_void, format_etc_in: *mut FormatEtc, medium: *mut StgMedium, release: i32)->HRESULT,
    query_get_data: unsafe extern "system" fn(this: *mut c_void, format_etc: *mut FormatEtc)->HRESULT,
    enum_formatetc: unsafe extern "system" fn(this: *mut c_void, direction: i32, enum_formatetc: *mut *mut IEnumFormatEtc)->HRESULT,
    advise: unsafe extern "system" fn(this: *mut c_void, format_etc: *mut FormatEtc, advf: i32, advsink: *mut IAdviseSink, connection: *mut i32)->HRESULT,
    unadvise: unsafe extern "system" fn(this: *mut c_void, connection: i32)->HRESULT,
    enum_advise: unsafe extern "system" fn(this: *mut c_void, ppenum_advise: *mut *mut IEnumSTATDATA)->HRESULT
}

static DATA_OBJECT_VTABLE: IDataObjectVTable = IDataObjectVTable {
    query_interface: IDataObject::query_interface,
    add_ref: IDataObject::add_ref,
    release: IDataObject::release,
    get_data: IDataObject::get_data,
    get_data_here: IDataObject::get_data_here,
    get_canonical_formatetc: IDataObject::get_canonical_formatetc,
    set_data: IDataObject::set_data,
    query_get_data: IDataObject::query_get_data,
    enum_formatetc: IDataObject::enum_formatetc,
    advise: IDataObject::advise,
    unadvise: IDataObject::unadvise,
    enum_advise: IDataObject::enum_advise
};

impl IDataObject {
    pub fn new(files: Vec<String>)->*mut Self {
        let dataobject = Box::new(IDataObject {
            vtbl: &DATA_OBJECT_VTABLE,
            ref_count: 1,
            files
        });   
        Box::into_raw(dataobject)
    }

    unsafe extern "system" fn query_interface(this: *mut c_void, riid: *const GUID, ppv: *mut *mut c_void) -> HRESULT {
        let self_ptr = this as *mut IDataObject;
        if *riid == IID_IUNKNOWN || *riid == IID_IDATAOBJECT {
            ((*self_ptr).vtbl.as_ref().unwrap().add_ref)(this);
            *ppv = self_ptr as *mut _;
            S_OK
        } else {
            *ppv = ptr::null_mut();
            E_NOINTERFACE
        }
    }

    unsafe extern "system" fn add_ref(this: *mut c_void) -> u32 {
        let self_ptr = this as *mut IDataObject;
        (*self_ptr).ref_count += 1;
        (*self_ptr).ref_count
    }

    unsafe extern "system" fn release(this: *mut c_void) -> u32 {
        let self_ptr = this as *mut IDataObject;
        (*self_ptr).ref_count -= 1;
        if (*self_ptr).ref_count == 0 {
            let _ = Box::from_raw(self_ptr);
        }
        (*self_ptr).ref_count
    }

    unsafe extern "system" fn get_data(this: *mut c_void, format_etc: *mut FormatEtc, medium: *mut StgMedium)->HRESULT {
        if (*format_etc).cf_format != 15 {
            return E_FAIL;
        }
        let self_ptr = this as *mut IDataObject;
        let files = 
            (*self_ptr).files
                .iter()
                .map(|f|string_to_pcwstr(&f))
                .fold(Vec::<u16>::new(), |a, c|[a, c].concat());

        let total_size = mem::size_of::<DropFiles>() + files.len() * 16;
        let global = GlobalAlloc(GHND, total_size).unwrap();

        let drop_files = GlobalLock(global) as *mut DropFiles;
        (*drop_files).files = std::mem::size_of::<DropFiles>() as i32;
        (*drop_files).wide = 1; 

        let data = (drop_files as *mut u8).add(std::mem::size_of::<DropFiles>()) as *mut u16;
        ptr::copy_nonoverlapping(files.as_ptr(), data, files.len());

        // Double null-terminate the file list.
        *data.add(files.len() - 1) = 0;

        // Unlock the global memory.
        let _ = GlobalUnlock(global);

        (*medium).tymed = 1;
        (*medium).global = global.0;
        (*medium).punk_for_release = null();
        S_OK
    }
    unsafe extern "system" fn get_data_here(_this: *mut c_void, _medium: *mut StgMedium)->HRESULT {
        return E_NOTIMPL
    }
    unsafe extern "system" fn get_canonical_formatetc(_this: *mut c_void, _format_etc_in: *mut FormatEtc, _format_etc_out: *mut FormatEtc)->HRESULT {
        return E_NOTIMPL
    }
    unsafe extern "system" fn set_data(_this: *mut c_void, _format_etc_in: *mut FormatEtc, _medium: *mut StgMedium, _release: i32)->HRESULT {
        return E_NOTIMPL
    }
    unsafe extern "system" fn query_get_data(_this: *mut c_void, format_etc: *mut FormatEtc)->HRESULT {
        if (*format_etc).cf_format == 15 {
            S_OK
        } else {
            E_FAIL
        }
    }
    unsafe extern "system" fn enum_formatetc(_this: *mut c_void, direction: i32, enum_formatetc: *mut *mut IEnumFormatEtc)->HRESULT {
        if direction != 1 {
            E_NOTIMPL
        } else {
            (*enum_formatetc) = IEnumFormatEtc::new();
            S_OK
        }
    }
    unsafe extern "system" fn advise(_this: *mut c_void, _format_etc: *mut FormatEtc, _advf: i32, _advsink: *mut IAdviseSink, _connection: *mut i32)->HRESULT {
        return E_NOTIMPL
    }
    unsafe extern "system" fn unadvise(_this: *mut c_void, _connection: i32)->HRESULT {
        return E_NOTIMPL
    }
    unsafe extern "system" fn enum_advise(_this: *mut c_void, _enum_advise: *mut *mut IEnumSTATDATA)->HRESULT {
        return E_NOTIMPL
    }
}
