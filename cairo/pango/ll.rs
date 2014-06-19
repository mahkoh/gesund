#![allow(non_camel_case_types)]
#![allow(dead_code)]

extern crate libc;

use cairo;
use cairo::pango;

pub struct PangoLayout;
pub struct PangoFontDescription;
pub static PANGO_SCALE: libc::c_double = 1024.0;

#[link(name = "pangocairo-1.0")]
#[link(name = "pango-1.0")]
extern {
    pub fn pango_font_description_new() -> *mut PangoFontDescription;
    pub fn pango_font_description_set_family(desc: *mut PangoFontDescription,
                                             family: *libc::c_char);
    pub fn pango_font_description_set_weight(desc: *mut PangoFontDescription,
                                             weight: pango::Weight);
    pub fn pango_font_description_set_absolute_size(desc: *mut PangoFontDescription,
                                                    size: libc::c_double);
    pub fn pango_font_description_free(desc: *mut PangoFontDescription);

    pub fn pango_layout_set_font_description(layout: *mut PangoLayout,
                                             dsc: *PangoFontDescription);
    pub fn pango_layout_set_text(layout: *mut PangoLayout, text: *libc::c_char,
                                 length: libc::c_int);
    pub fn pango_layout_get_extents(layout: *mut PangoLayout,
                                    ink_rect: *mut pango::Rectangle,
                                    logical_rectangle: *mut pango::Rectangle);
    pub fn pango_layout_set_width(layout: *mut PangoLayout, width: libc::c_int);

    pub fn pango_cairo_create_layout(cr: *mut cairo::ll::cairo_t) -> *mut PangoLayout;
    pub fn pango_cairo_show_layout(cr: *mut cairo::ll::cairo_t, layout: *mut PangoLayout);
}
