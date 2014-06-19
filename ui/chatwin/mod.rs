use state::{State};
use utils::{RefMut, CopyMut};
use assets::{Cached};
use cairo::{Surface};
use cairo::{pango};
use cairo::pango::{FontDescription};
use colors;
use ui;

pub struct Chatwin<'a> {
    pub state: RefMut<State<'a>>,
    pub assets: RefMut<Cached<'a>>,
    pub height: CopyMut<f64>,
    pub width: CopyMut<f64>,
    pub scale: CopyMut<f64>,
    pub num: Option<uint>,
}

pub static LEFT: f64 = ui::sidebar::WIDTH;
pub static HEADER_HEIGHT: f64 = ui::sidebar::HEADER_HEIGHT + 1f64;
pub static TEXTBOX_HEIGHT: f64 = 60f64;

macro_rules! scale {
    ($x:expr) => { self.scale.get() * $x }
}

impl<'a> Chatwin<'a> {
    pub fn draw_all(&self, surface: &mut Surface) {
        if self.num.is_none() {
            return;
        }

        // temporary
        {
            let mut surface = surface.create_for_rectangle(scale!(LEFT), 0.0,
                                                           self.width.get(),
                                                           self.height.get());
            let mut cx = surface.create();
            cx.set_source_rgb(colors::WHITE);
            cx.paint();
        }
        self.draw_header(surface);
        self.draw_textbox(surface);
    }

    pub fn draw_textbox(&self, surface: &mut Surface) {
        let top = self.height.get() - scale!(TEXTBOX_HEIGHT + 10.0);
        let width = self.width.get() - scale!(LEFT + 105.0);
        let mut surface = surface.create_for_rectangle(scale!(LEFT+10.0), top, width,
                                                       scale!(TEXTBOX_HEIGHT));
        let state = self.state.borrow();
        state.friends.get(self.num.unwrap()).textbox.draw(width, &mut surface);
    }

    pub fn draw_header(&self, surface: &mut Surface) {
        if self.num.is_none() {
            return;
        }
        let state = self.state.borrow();
        let friend = state.friends.get(self.num.unwrap());

        let width = self.width.get() - scale!(LEFT);
        let mut surface = surface.create_for_rectangle(scale!(LEFT), 0.0, width,
                                                       scale!(HEADER_HEIGHT));
        let mut cx = surface.create();

        // Draw background
        cx.set_source_rgb(colors::WHITE);
        cx.paint();

        // Draw avatar
        match friend.avatar {
            Some(ref a) => {
                cx.set_source_surface(a, scale!(10.0), scale!(10.0));
                cx.paint();
            },
            None => {
                let assets = self.assets.borrow();
                cx.set_source_surface(&assets.group_large_black, scale!(10.0),
                                      scale!(10.0));
                cx.paint();
            }
        }

        // Draw line
        cx.move_to(0.0, scale!(HEADER_HEIGHT - 1.0));
        cx.set_line_width(scale!(2.0));
        cx.set_dash([scale!(2.0)], scale!(2.5));
        cx.rel_line_to(width, 0.0);
        cx.set_source_rgb(colors::LIGHT_GREY);
        cx.stroke();

        // Draw name and status
        let mut font = FontDescription::new();
        font.set_family("sans");
        font.set_weight(pango::WeightBold);
        font.set_absolute_size(scale!(14.0));
        let mut layout = cx.create_pango_layout();
        layout.set_font_description(&font);
        layout.set_text(friend.name.as_slice());
        cx.set_source_rgb(colors::DARK_GREY);
        cx.move_to(scale!(60.0), scale!(15.0));
        cx.show_pango_layout(&layout);

        font.set_weight(pango::WeightNormal);
        font.set_absolute_size(scale!(11.0));
        layout.set_font_description(&font);
        layout.set_text(friend.status.as_slice());
        cx.set_source_rgb(colors::MEDIUM_GREY);
        cx.move_to(scale!(60.0), scale!(33.0));
        cx.show_pango_layout(&layout);
    }
}
