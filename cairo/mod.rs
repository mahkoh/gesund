#![feature(globs)]

use libc::{c_int, c_uint};

use self::ll::*;
use colors::{RGB};

use std::f64::consts::{FRAC_PI_2};

mod ll;
mod glib;
pub mod pango;
pub mod svg;

#[repr(C)]
pub enum Antialias {
    AntialiasDefault  = 0,
    AntialiasNone     = 1,
    AntialiasGray     = 2,
    AntialiasSubpixel = 3,
    AntialiasFast     = 4,
    AntialiasGood     = 5,
    AntialiasBest     = 6,
}

#[repr(C)]
pub enum LineCap {
    LineCapButt = CAIRO_LINE_CAP_BUTT,
    LineCapRound = CAIRO_LINE_CAP_ROUND,
    LineCapSquare = CAIRO_LINE_CAP_SQUARE,
}

pub struct Cairo<'a> {
    raw: *mut cairo_t,
}

impl<'a> Cairo<'a> {
    pub fn set_line_width(&mut self, width: f64) {
        unsafe { cairo_set_line_width(self.raw, width); }
    }

    pub fn new_path(&mut self) {
        unsafe { cairo_new_path(self.raw); }
    }

    pub fn set_source_rgb(&mut self, rgb: RGB) {
        unsafe { cairo_set_source_rgb(self.raw, rgb.r, rgb.g, rgb.b); }
    }

    pub fn arc(&mut self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) {
        unsafe { cairo_arc(self.raw, xc, yc, radius, angle1, angle2); }
    }

    pub fn rectangle(&mut self, x: f64, y: f64, width: f64, height: f64) {
        unsafe { cairo_rectangle(self.raw, x, y, width, height); }
    }

    pub fn get_target<'b>(&'b mut self) -> Surface<'b> {
        let raw = unsafe { cairo_get_target(self.raw) };
        assert!(!raw.is_null());
        Surface { raw: raw }
    }

    pub fn clip(&mut self) {
        unsafe { cairo_clip(self.raw); }
    }

    pub fn set_antialias(&mut self, val: Antialias) {
        unsafe { cairo_set_antialias(self.raw, val as c_uint); }
    }

    pub fn rounded_rectangle(&mut self, x: f64, y: f64, width: f64, height: f64,
                             radius: f64) {
        unsafe {
            cairo_new_path(self.raw);
            cairo_arc(self.raw, x + radius, y + radius, radius,
                      2.0 * FRAC_PI_2, 3.0 * FRAC_PI_2);
            cairo_arc(self.raw, x + width - radius, y + radius, radius,
                      3.0 * FRAC_PI_2, 0.0 * FRAC_PI_2);
            cairo_arc(self.raw, x + width - radius, y + height - radius, radius,
                      0.0 * FRAC_PI_2, 1.0 * FRAC_PI_2);
            cairo_arc(self.raw, x + radius, y + height - radius, radius,
                      1.0 * FRAC_PI_2, 2.0 * FRAC_PI_2);
            cairo_close_path(self.raw);
        }
    }

    pub fn stroke(&mut self) {
        unsafe { cairo_stroke(self.raw); }
    }

    pub fn fill(&mut self) {
        unsafe { cairo_fill(self.raw); }
    }

    pub fn paint(&mut self) {
        unsafe { cairo_paint(self.raw); }
    }

    pub fn create_pango_layout(&self) -> pango::Layout {
        let raw = unsafe { pango::ll::pango_cairo_create_layout(self.raw) };
        assert!(!raw.is_null());
        pango::Layout { raw: raw }
    }

    pub fn show_pango_layout(&mut self, layout: &pango::Layout) {
        unsafe { pango::ll::pango_cairo_show_layout(self.raw, layout.raw); }
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        unsafe { cairo_move_to(self.raw, x, y); }
    }

    pub fn line_to(&mut self, x: f64, y: f64) {
        unsafe { cairo_line_to(self.raw, x, y); }
    }

    pub fn set_line_cap(&mut self, cap: LineCap) {
        unsafe { cairo_set_line_cap(self.raw, cap as c_uint); }
    }

    pub fn rel_line_to(&mut self, dx: f64, dy: f64) {
        unsafe { cairo_rel_line_to(self.raw, dx, dy); }
    }

    pub fn close_path(&mut self) {
        unsafe { cairo_close_path(self.raw); }
    }

    pub fn set_fill_rule(&mut self, rule: FillRule) {
        unsafe { cairo_set_fill_rule(self.raw, rule as c_uint); }
    }

    pub fn set_source_surface(&mut self, source: &Surface, x: f64, y: f64) {
        unsafe { cairo_set_source_surface(self.raw, source.raw, x, y); }
    }

    pub fn scale(&mut self, sx: f64, sy: f64) {
        unsafe { cairo_scale(self.raw, sx, sy); }
    }
}

impl<'a> Drop for Cairo<'a> {
    fn drop(&mut self) {
        unsafe { cairo_destroy(self.raw) }
    }
}

/*
pub struct Pattern<'a> {
    raw: *mut cairo_pattern_t,
}

impl<'a> Pattern<'a> {
}
*/

pub struct Surface<'a> {
    raw: *mut cairo_surface_t,
}

impl<'a> Surface<'a> {
    pub fn create<'a>(&'a mut self) -> Cairo<'a> {
        let raw = unsafe { cairo_create(self.raw) };
        assert!(!raw.is_null());
        Cairo { raw: raw }
    }

    pub fn create_for_rectangle<'b>(&'b mut self, x: f64, y: f64, width: f64,
                                    height: f64) -> Surface<'b> {
        let raw = unsafe { cairo_surface_create_for_rectangle(self.raw, x, y, width,
                                                              height) };
        assert!(!raw.is_null());
        Surface { raw: raw }
    }

    pub fn write_to_png(&self, path: &Path) {
        // XXX return value
        unsafe {
            path.with_c_str(|p| cairo_surface_write_to_png(self.raw, p));
        }
    }
}

impl<'a> Drop for Surface<'a> {
    fn drop(&mut self) {
        unsafe { cairo_surface_destroy(self.raw); }
    }
}

pub fn pdf_surface_create(dst: &Path, width: f64, height: f64) -> Surface {
    let raw = unsafe {
        dst.with_c_str(|p| cairo_pdf_surface_create(p, width, height))
    };
    Surface { raw: raw }
}

#[repr(C)]
pub enum Format {
    FormatInvalid   = ll::CAIRO_FORMAT_INVALID,
    FormatArgb32    = ll::CAIRO_FORMAT_ARGB32,
    FormatRgb24     = ll::CAIRO_FORMAT_RGB24,
    FormatA8        = ll::CAIRO_FORMAT_A8,
    FormatA1        = ll::CAIRO_FORMAT_A1,
    FormatRgb16_565 = ll::CAIRO_FORMAT_RGB16_565,
    FormatRgb30     = ll::CAIRO_FORMAT_RGB30,
}

#[repr(C)]
pub enum FillRule {
    Winding = ll::CAIRO_FILL_RULE_WINDING,
    EvenOdd = ll::CAIRO_FILL_RULE_EVEN_ODD,
}

#[repr(C)]
pub enum Content {
    ContentColor = ll::CAIRO_CONTENT_COLOR,
    ContentAlpha = ll::CAIRO_CONTENT_ALPHA,
    ContentColorAlpha = ll::CAIRO_CONTENT_COLOR_ALPHA,
}

pub fn image_surface_create(format: Format, width: i32, height: i32) -> Surface {
    let raw = unsafe { cairo_image_surface_create(format as c_int, width, height) };
    assert!(!raw.is_null());
    Surface { raw: raw }
}

pub fn recording_surface_create(content: Content) -> Surface {
    let raw = unsafe { cairo_recording_surface_create(content as c_uint, 0 as *_) };
    assert!(!raw.is_null());
    Surface { raw: raw }
}

pub fn image_surface_create_for_data(data: &mut [u8], format: Format, width: i32,
                                     height: i32, stride: i32) -> Surface {
    let raw = unsafe {
        cairo_image_surface_create_for_data(data.as_mut_ptr(), format as c_int, width,
                                            height, stride)
    };
    assert!(!raw.is_null());
    Surface { raw: raw }
}
