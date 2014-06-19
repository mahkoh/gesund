use cairo::{Surface};

pub trait Drawable {
    fn width(&self) -> f64;
    fn height(&self) -> f64;
    fn draw_to(&self, &mut Surface);
}
