use cairo::{Surface, pango};
use cairo::pango::{};
use utils::{CopyMut};
use colors;

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

    pub fn draw(&self, width: f64, surface: &mut Surface) {
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
}
