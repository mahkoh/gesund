extern crate libc;

use cairo::ll::{cairo_t};
use cairo::{glib};

pub struct RsvgHandle;
pub struct RsvgDimensionData {
    pub width: libc::c_int,
    pub height: libc::c_int,
    em: libc::c_double,
    ex: libc::c_double,
}

#[link(name = "rsvg-2")]
extern {
    pub fn rsvg_handle_new_from_file(file_name: *libc::c_char,
                                     error: *mut *mut glib::GError) -> *mut RsvgHandle;
    pub fn rsvg_handle_new_from_data(data: *u8, size: libc::size_t, 
                                     error: *mut *mut glib::GError) -> *mut RsvgHandle;
    pub fn rsvg_handle_get_dimensions(handle: *mut RsvgHandle,
                                      dimension_data: *mut RsvgDimensionData);
    pub fn rsvg_handle_render_cairo(handle: *mut RsvgHandle,
                                    cr: *mut cairo_t) -> libc::c_int;
}
