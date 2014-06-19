extern crate libc;

pub struct GError;

#[link(name = "gobject-2.0")]
#[link(name = "glib-2.0")]
extern {
    pub fn g_object_unref(ptr: *mut libc::c_void);
}
