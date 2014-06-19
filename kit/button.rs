use cairo::{Surface, pango, RGB};
use cairo::pango::{FontDescription};
use kit::drawable::{Drawable};

pub struct Button {
    pub width: f64,
    pub height: f64,
    pub caption: String,
}

impl Button {
    pub fn new(width: f64, height: f64, caption: &str) -> Button {
        Button {
            width: width,
            height: height,
            caption: caption.to_string(),
        }
    }
}

impl Drawable for Button {
    fn width(&self) -> f64 {
        self.width
    }

    fn height(&self) -> f64 {
        self.height
    }

    fn draw_to(&self, surface: &mut Surface) {
        let mut cx = surface.create();

        let mut font = FontDescription::new();
        font.set_family("sans");
        font.set_weight(pango::WeightNormal);
        font.set_absolute_size(35.0);
        let mut layout = cx.create_pango_layout();
        layout.set_font_description(&font);
        layout.set_text(self.caption.as_slice());
        let (width, height) = layout.get_extends();

        cx.move_to((self.width - width)/2.0, (self.height - height)/2.0);
        cx.set_source_rgb(RGB(0.0, 0.0, 1.0));
        cx.show_pango_layout(&layout);

        cx.set_line_width(2.0);
        cx.set_source_rgb(RGB(0.0, 0.0, 0.0));
        cx.rectangle(0.0, 0.0, self.width, self.height);
        cx.stroke();
    }
}
