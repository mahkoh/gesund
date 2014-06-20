use cairo::{Surface, pango};
use cairo::pango::{FontDescription};
use utils::{CopyMut};
use colors;

pub static PADDING: f64 = 10f64;

macro_rules! scale {
    ($x:expr) => { self.scale.get() * $x }
}

pub struct Textbox {
    pub text: String,
    pub cursor: uint,
    pub scroll: f64,
    pub empty_text: Option<String>,
    pub height: f64,
    pub scale: CopyMut<f64>,
}

impl Textbox {
    pub fn new(height: f64, scale: CopyMut<f64>) -> Textbox {
        Textbox {
            text: String::new(),
            cursor: 0,
            scroll: 0.0,
            empty_text: None,
            height: height,
            scale: scale,
        }
    }

    /*
    pub fn click(&self, x: f64, y: f64) {
        let mut font = FontDescription::new();
        font.set_family("sans");
        font.set_weight(pango::WeightNormal);
        font.set_absolute_size(scale!(12.0));
        let mut layout = cx.create_pango_layout();
        layout.set_font_description(&font);
        layout.set_text(self.text.as_slice());
        layout.set_width(width - scale!(2*PADDING));
    }
    */

    pub fn draw(&self, width: f64, surface: &mut Surface) {
        {
            let mut cx = surface.create();

            // Background
            cx.set_source_rgb(colors::WHITE);
            cx.paint();

            // Border
            cx.rectangle(scale!(1.0), scale!(1.0), width - scale!(2.0),
                         scale!(self.height - 2.0));
            cx.set_source_rgb(colors::LIGHT_GREY);
            cx.set_line_width(scale!(2.0));
            cx.stroke();
        }
        {
            let mut surface = surface.create_for_rectangle(scale!(PADDING),
                                                           scale!(PADDING),
                                                       width - scale!(2.0*PADDING),
                                                       scale!(self.height - 2.0*PADDING));
            let mut cx = surface.create();

            // Text
            let mut font = FontDescription::new();
            font.set_family("sans");
            font.set_weight(pango::WeightNormal);
            font.set_absolute_size(scale!(12.0));
            let mut layout = cx.create_pango_layout();
            layout.set_font_description(&font);
            layout.set_text(self.text.as_slice());
            layout.set_width(width - scale!(2.0*PADDING));
            cx.set_source_rgb(colors::BLACK);
            cx.show_pango_layout(&layout);
        }
    }
}
