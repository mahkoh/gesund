use cairo::{Surface};
use kit::drawable::{Drawable};

pub struct Container {
    pub width: f64,
    pub height: f64,
    pub elements: Vec<Element>,
}

impl Container {
    pub fn new(width: f64, height: f64) -> Container {
        Container {
            width: width,
            height: height,
            elements: Vec::new(),
        }
    }

    pub fn add_at(&mut self, x: f64, y: f64, raw: Box<Drawable>) {
        self.elements.push(Element {
            x: x,
            y: y,
            raw: raw
        });
    }
}

impl Drawable for Container {
    fn width(&self) -> f64 {
        self.width
    }

    fn height(&self) -> f64 {
        self.height
    }

    fn draw_to(&self, surface: &mut Surface) {
        for element in self.elements.iter() {
            element.draw_to(surface);
        }
    }
}

pub struct Element {
    pub x: f64,
    pub y: f64,
    pub raw: Box<Drawable>,
}

impl Drawable for Element {
    fn width(&self) -> f64 {
        self.raw.width()
    }

    fn height(&self) -> f64 {
        self.raw.height()
    }

    fn draw_to(&self, surface: &mut Surface) {
        let mut sub = surface.create_for_rectangle(self.x, self.y, self.width(),
                                                   self.height());
        self.raw.draw_to(&mut sub);
    }
}
