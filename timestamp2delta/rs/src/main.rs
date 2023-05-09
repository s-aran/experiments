// use chrono::{self, DateTime, Local, NaiveTime};
use windows::Win32::{
    Foundation::{FALSE, TRUE},
    System::Ole::CF_UNICODETEXT,
};

use crate::win_cb::{Clipboard, GlobalMemory};

mod win_cb {
    use windows::Win32::{
        Foundation::{GetLastError, ERROR_SUCCESS, FALSE, HANDLE, HGLOBAL, HWND, TRUE},
        System::{
            DataExchange::{
                CloseClipboard, EmptyClipboard, GetClipboardData, IsClipboardFormatAvailable,
                OpenClipboard, SetClipboardData,
            },
            Memory::{
                GlobalAlloc, GlobalFree, GlobalLock, GlobalSize, GlobalUnlock, GMEM_MOVEABLE,
                GMEM_ZEROINIT,
            },
            Ole::CLIPBOARD_FORMAT,
        },
    };

    pub struct Clipboard {
        opened: bool,
        clipboard_format: CLIPBOARD_FORMAT,
    }

    impl Clipboard {
        pub fn new(clipboard_format: CLIPBOARD_FORMAT) -> Self {
            Self {
                opened: false,
                clipboard_format,
            }
        }

        pub fn type_of(&self) -> bool {
            unsafe { IsClipboardFormatAvailable(self.clipboard_format.0.into()) == TRUE }
        }

        pub fn open(&mut self) -> Result<(), String> {
            if self.is_opened() {
                return Err("Clipboard already opened.".to_owned());
            }

            if unsafe { OpenClipboard(HWND(0)) } == FALSE {
                return Err("OpenClipboard() failed.".to_owned());
            }

            self.opened = true;
            Ok(())
        }

        pub fn close(&mut self) -> Result<(), String> {
            if !self.is_opened() {
                return Err("Clipboard already closed.".to_owned());
            }

            if unsafe { CloseClipboard() } == FALSE {
                return Err("CloseClipboard() failed.".to_owned());
            }

            self.opened = false;
            Ok(())
        }

        pub fn is_opened(&self) -> bool {
            self.opened
        }

        pub fn get_clipboard_memory(&self) -> Result<HANDLE, String> {
            if !self.is_opened() {
                return Err("Clipboard is not opened.".to_owned());
            }

            match unsafe { GetClipboardData(self.clipboard_format.0.into()) } {
                Ok(h) => return Ok(h),
                Err(e) => return Err(format!("GetClipboardData() failed: {}", e.to_string())),
            };
        }

        pub fn set_data(&self, h_global: HGLOBAL) -> Result<(), String> {
            if !self.is_opened() {
                return Err("Clipboard is not opened.".to_owned());
            }

            match unsafe { SetClipboardData(self.clipboard_format.0.into(), HANDLE(h_global.0)) } {
                Ok(_) => return Ok(()),
                Err(e) => return Err(format!("SetClipboardData() failed: {}", e.to_string())),
            }
        }

        pub fn empty(&self) -> Result<(), String> {
            if !self.is_opened() {
                return Err("Clipboard is not opened.".to_owned());
            }

            if unsafe { EmptyClipboard() } == FALSE {
                return Err("EmptyClipboard() failed.".to_owned());
            }

            Ok(())
        }
    }

    impl Drop for Clipboard {
        fn drop(&mut self) {
            match self.close() {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
    }

    pub struct GlobalMemory {
        handle: HANDLE,
        allocated: bool,
        locked: bool,
        h_global: HGLOBAL,
        ptr: *mut std::ffi::c_void,
    }

    impl GlobalMemory {
        pub fn new() -> Self {
            Self {
                handle: HANDLE(0),
                h_global: HGLOBAL(0),
                ptr: std::ptr::null_mut(),
                allocated: false,
                locked: false,
            }
        }

        pub fn get_handle(&self) -> HANDLE {
            self.handle
        }

        pub fn get_global(&self) -> HGLOBAL {
            self.h_global
        }

        pub fn is_allocated(&self) -> bool {
            self.allocated
        }

        pub fn is_locked(&self) -> bool {
            self.locked
        }

        pub fn size(&self) -> usize {
            unsafe { GlobalSize(self.h_global) }
        }

        pub fn alloc(&mut self, size: usize) -> Result<*mut std::ffi::c_void, String> {
            if self.is_locked() {
                return Err("already locked.".to_owned());
            }

            if self.is_allocated() {
                return Err("already allocated.".to_owned());
            }

            let h_global = match unsafe { GlobalAlloc(GMEM_MOVEABLE | GMEM_ZEROINIT, size) } {
                Ok(h) => h,
                Err(e) => return Err(format!("GlobalAlloc() failed: {}", e.to_string())),
            };

            self.allocated = true;

            self.lock(h_global)
        }

        pub fn alloc_without_free(&mut self, size: usize) -> Result<*mut std::ffi::c_void, String> {
            let result = self.alloc(size);

            // avoid call free() pm drop
            self.allocated = false;

            result
        }

        pub fn lock(&mut self, h_global: HGLOBAL) -> Result<*mut std::ffi::c_void, String> {
            if self.is_locked() {
                return Err("already locked.".to_owned());
            }

            self.ptr = unsafe { GlobalLock(h_global) };
            if self.ptr.is_null() {
                return Err("GlobalLock() failed.".to_owned());
            }

            self.locked = true;
            self.h_global = h_global;

            Ok(self.ptr)
        }

        pub fn lock_by_handle(&mut self, handle: HANDLE) -> Result<*mut std::ffi::c_void, String> {
            self.lock(HGLOBAL(handle.0))
        }

        pub fn unlock(&mut self) -> Result<(), String> {
            if !self.is_locked() {
                return Err("never locked.".to_owned());
            }

            if unsafe { GlobalUnlock(self.h_global) } == FALSE
                && unsafe { GetLastError() } != ERROR_SUCCESS
            {
                return Err("GlobalUnlock() failed.".to_owned());
            }

            self.locked = false;
            self.ptr = std::ptr::null_mut();

            Ok(())
        }

        pub fn free(&mut self) -> Result<(), String> {
            if !self.is_allocated() {
                return Err("never alloced.".to_owned());
            }

            match unsafe { GlobalFree(self.h_global) } {
                Ok(_) => {
                    self.allocated = false;
                    self.h_global = HGLOBAL(0);
                    self.ptr = std::ptr::null_mut();
                }
                Err(e) => {
                    return Err(format!("GlobalFree() failed: {}", e.to_string()));
                }
            };

            Ok(())
        }
    }

    impl Drop for GlobalMemory {
        fn drop(&mut self) {
            // avoid error
            if self.is_locked() {
                match self.unlock() {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{}", e);
                        return;
                    }
                }
            }

            // avoid error
            if self.is_allocated() {
                match self.free() {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{}", e);
                        return;
                    }
                }
            }
        }
    }
}

pub fn get_text_from_clipboard() -> Result<String, ()> {
    let mut clipboard = Clipboard::new(CF_UNICODETEXT);

    let is_clipboard_format_available = clipboard.type_of();
    println!(
        "Is clipborad format available? => {}",
        if is_clipboard_format_available == TRUE {
            "Yes"
        } else {
            "No"
        }
    );

    if is_clipboard_format_available == FALSE {
        eprintln!("Invalid clipboard type.");
        return Err(());
    }

    if clipboard.open().is_err() {
        eprintln!("Failed to open clipboard.");
        return Err(());
    }

    let h_global = match clipboard.get_clipboard_memory() {
        Ok(h) => h,
        Err(e) => {
            eprintln!("{}", e);
            return Err(());
        }
    };

    let mut mem = GlobalMemory::new();
    let data = match mem.lock_by_handle(h_global) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("{}", e);
            return Err(());
        }
    };

    let str_slice = unsafe { std::slice::from_raw_parts(data as *mut u16, (mem.size() + 1) / 2) };
    let result = String::from_utf16_lossy(str_slice);

    Ok(result)
}

pub fn set_text_to_clipboard(text: &String) -> Result<(), ()> {
    let mut clipboard = Clipboard::new(CF_UNICODETEXT);

    match clipboard.open() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e.to_owned());
            return Err(());
        }
    }

    match clipboard.empty() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e);
            return Err(());
        }
    }

    let src: Vec<u16> = text.encode_utf16().collect();
    let global_size = (src.len() + 1) * std::mem::size_of::<u16>();

    // for e in src.iter() {
    //     println!("0x{:04X}", e);
    // }

    let mut mem = GlobalMemory::new();
    let ptr = match mem.alloc_without_free(global_size) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            return Err(());
        }
    };

    unsafe { std::ptr::copy(src.as_ptr(), ptr as *mut u16, src.len()) };

    // for i in 0..(src.len() + 1) {
    //     unsafe { println!("0x{:04X}", (*(ptr as *mut u16).offset(i as isize))) };
    // }

    match mem.unlock() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e);
            return Err(());
        }
    };

    match clipboard.set_data(mem.get_global()) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e);
            return Err(());
        }
    };

    Ok(())
}

fn main() {
    println!("Hello, world!");

    let t = match get_text_from_clipboard() {
        Ok(t) => t,
        Err(()) => {
            return ();
        }
    };

    println!("{}", t);

    let t2 = match get_text_from_clipboard() {
        Ok(t) => t,
        Err(()) => {
            return ();
        }
    };

    println!("{}", t2);

    // match set_text_to_clipboard(&"ああああaあ".to_string()) {
    //     Ok(()) => {}
    //     Err(()) => {
    //         eprintln!("set clipboard error.");
    //         return ();
    //     }
    // };

    match set_text_to_clipboard(&"🍣🍺".to_string()) {
        Ok(()) => {}
        Err(()) => {
            eprintln!("set clipboard error.");
            return ();
        }
    };

    // let format = "%H:%M";
    // let a_dt: DateTime<Local> = match Local.datetime_from_str("14:00", &format) {
    //     Ok(d) => d,
    //     Err(e) => {
    //         eprintln!("err: {}", e.to_string());
    //         return ();
    //     }
    // };

    // let b_dt: Time<Local> = match Local.datetime_from_str("10:00", &format) {
    //     Ok(d) => d,
    //     Err(e) => {
    //         eprintln!("err: {}", e.to_string());
    //         return ();
    //     }
    // };

    // println!("{} - {} = {}", a_dt, b_dt, a_dt - b_dt);
}
