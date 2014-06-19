#![feature(macro_rules)]

pub struct RGB {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

macro_rules! rgb {
    ($c:expr) => {
        RGB { r: ((($c >> 16) & 255) as f64 / 255.0),
              g: ((($c >> 8 ) & 255) as f64 / 255.0),
              b: ((($c      ) & 255) as f64 / 255.0) }
    }
}

pub static GREEN:       RGB = rgb!(0x6bc260);
pub static YELLOW:      RGB = rgb!(0xcebf44);
pub static RED:         RGB = rgb!(0xc84e4e);
pub static BLACK:       RGB = rgb!(0x000000);
pub static DARK_GREY:   RGB = rgb!(0x1c1c1c);
pub static MEDIUM_GREY: RGB = rgb!(0x414141);
pub static LIGHT_GREY:  RGB = rgb!(0xd1d1d1);
pub static WHITE:       RGB = rgb!(0xffffff);

