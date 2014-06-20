#![allow(non_camel_case_types)]
#![allow(dead_code)]

extern crate libc;

use libc::{c_int, c_char, c_double};

use cairo;
use cairo::pango;

pub struct PangoLayout;
pub struct PangoFontDescription;
pub static PANGO_SCALE: c_double = 1024.0;

#[link(name = "pangocairo-1.0")]
#[link(name = "pango-1.0")]
extern {
    pub fn pango_font_description_new() -> *mut PangoFontDescription;
    pub fn pango_font_description_set_family(desc: *mut PangoFontDescription,
                                             family: *c_char);
    pub fn pango_font_description_set_weight(desc: *mut PangoFontDescription,
                                             weight: pango::Weight);
    pub fn pango_font_description_set_absolute_size(desc: *mut PangoFontDescription,
                                                    size: c_double);
    pub fn pango_font_description_free(desc: *mut PangoFontDescription);

    pub fn pango_layout_set_font_description(layout: *mut PangoLayout,
                                             dsc: *PangoFontDescription);
    pub fn pango_layout_set_text(layout: *mut PangoLayout, text: *c_char,
                                 length: c_int);
    pub fn pango_layout_get_extents(layout: *mut PangoLayout,
                                    ink_rect: *mut pango::Rectangle,
                                    logical_rectangle: *mut pango::Rectangle);
    pub fn pango_layout_set_width(layout: *mut PangoLayout, width: c_int);
    pub fn pango_layout_xy_to_index(layout: *mut PangoLayout, x: c_int, y: c_int,
                                    index: *mut c_int, trailing: *mut c_int);

    pub fn pango_cairo_create_layout(cr: *mut cairo::ll::cairo_t) -> *mut PangoLayout;
    pub fn pango_cairo_show_layout(cr: *mut cairo::ll::cairo_t, layout: *mut PangoLayout);
}
