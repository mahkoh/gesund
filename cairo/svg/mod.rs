use std;
use libc;
use cairo::{glib, Cairo};

use self::ll::*;

mod ll;

pub struct Handle {
    raw: *mut RsvgHandle,
}

impl Handle {
    /*
    pub fn from_file(path: &Path) -> Option<Handle> {
        let raw = path.with_c_str(|p| unsafe { 
            rsvg_handle_new_from_file(p, 0 as *mut _)
        });
        if raw.is_null() {
            None
        } else {
            Some(Handle { raw: raw })
        }
    }
    */

    pub fn from_slice(data: &[u8]) -> Option<Handle> {
        let raw = unsafe {
            rsvg_handle_new_from_data(data.as_ptr(), data.len() as libc::size_t,
                                      0 as *mut _)
        };
        if raw.is_null() {
            None
        } else {
            Some(Handle { raw: raw })
        }
    }

    pub fn dimensions(&self) -> (f64, f64) {
        let mut dim = unsafe { std::mem::uninitialized() };
        unsafe { rsvg_handle_get_dimensions(self.raw, &mut dim as *mut _); }
        (dim.width as f64, dim.height as f64)
    }

    pub fn render_cairo(&self, cx: Cairo) -> bool {
        unsafe { rsvg_handle_render_cairo(self.raw, cx.raw) != 0 }
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { glib::g_object_unref(self.raw as *mut _); }
    }
}
