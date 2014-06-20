use self::ll::*;
use libc::{c_int};
use cairo::{glib};
use std;

pub mod ll;

#[repr(C)]
pub enum Weight {
    WeightThin       = 100,
    WeightUltralight = 200,
    WeightLight      = 300,
    WeightBook       = 380,
    WeightNormal     = 400,
    WeightMedium     = 500,
    WeightSemibold   = 600,
    WeightBold       = 700,
    WeightUltrabold  = 800,
    WeightHeavy      = 900,
    WeightUltraheavy = 1000
}

pub struct Rectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

pub struct FontDescription {
    raw: *mut PangoFontDescription,
}

impl FontDescription {
    pub fn new() -> FontDescription {
        let raw = unsafe { pango_font_description_new() };
        FontDescription { raw: raw }
    }

    pub fn set_family(&mut self, family: &str) {
        family.with_c_str(|p| {
            unsafe { pango_font_description_set_family(self.raw, p); }
        });
    }

    pub fn set_weight(&mut self, weight: Weight) {
        unsafe { 
            pango_font_description_set_weight(self.raw, weight);
        }
    }

    pub fn set_absolute_size(&mut self, size: f64) {
        unsafe {
            pango_font_description_set_absolute_size(self.raw, size * PANGO_SCALE);
        }
    }
}

impl Drop for FontDescription {
    fn drop(&mut self) {
        unsafe { pango_font_description_free(self.raw); }
    }
}

pub struct Layout<'a> {
    pub raw: *mut PangoLayout,
}

impl<'a> Layout<'a> {
    pub fn set_font_description(&mut self, desc: &'a FontDescription) {
        unsafe { pango_layout_set_font_description(self.raw, desc.raw as *_); }
    }

    pub fn set_text(&mut self, text: &str) {
        unsafe {
            pango_layout_set_text(self.raw, text.as_ptr() as *_, text.len() as c_int);
        }
    }

    pub fn get_extends(&self) -> (f64, f64) {
        let mut rec = unsafe { std::mem::uninitialized() };
        unsafe {
            pango_layout_get_extents(self.raw, 0 as *mut _, &mut rec as *mut _);
        }
        (rec.width as f64 / ll::PANGO_SCALE,
         rec.height as f64 / ll::PANGO_SCALE)
    }

    pub fn set_width(&mut self, width: f64) {
        unsafe { pango_layout_set_width(self.raw, (width*ll::PANGO_SCALE) as c_int); }
    }

    pub fn xy_to_line(&mut self, x: f64, y: f64) -> uint {
        let mut index: c_int = 0;
        let mut trailing: c_int = 0;
        unsafe { pango_layout_xy_to_index(self.raw, x as c_int, y as c_int,
                                          &mut index as *mut c_int,
                                          &mut trailing as *mut c_int); }
        index as uint
    }
}

impl<'a> Drop for Layout<'a> {
    fn drop(&mut self) {
        unsafe { glib::g_object_unref(self.raw as *mut _); }
    }
}
