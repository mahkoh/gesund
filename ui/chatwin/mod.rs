use state::{State, TextMessage, FileMessage};
use utils::{RefMut, CopyMut};
use assets::{Cached};
use cairo::{Surface};
use cairo::{pango};
use cairo::pango::{FontDescription, AlignLeft, AlignRight};
use colors;
use ui;
// use ui::{textbox};
use std::f64::consts::{FRAC_PI_2};

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

pub static BIG_BUTTON_WIDTH: f64 = 50f64;
pub static BIG_BUTTON_HEIGHT: f64 = 40f64;
// pub static SMALL_BUTTON_WIDTH: f64 = 22f64;
pub static PADDING: f64 = 10f64;
pub static RADIUS: f64 = 5f64;
pub static BIG_ICON_SIZE: f64 = 23f64;
// pub static SMALL_ICON_SIZE: f64 = 14f64;

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
        self.draw_messages(surface);
    }

    pub fn draw_textbox(&self, surface: &mut Surface) {
        let assets = self.assets.borrow();

        let top = self.height.get() - scale!(TEXTBOX_HEIGHT + 10.0);
        let width = self.width.get() - scale!(LEFT + 100.0);
        {
            let mut surface = surface.create_for_rectangle(scale!(LEFT+10.0), top, width,
                                                           scale!(TEXTBOX_HEIGHT));
            let state = self.state.borrow();
            state.friends.get(self.num.unwrap()).textbox.draw(width, &mut surface);
        }

        let mut cx = surface.create();
        cx.move_to(scale!(LEFT+8.0)+width, top);
        cx.rel_line_to(scale!(17.0), 0.0);
        cx.arc(scale!(LEFT+25.0)+width, top+scale!(5.0), scale!(5.0), -FRAC_PI_2, 0.0);  
        cx.rel_line_to(0.0, scale!(TEXTBOX_HEIGHT/2.0 - 6.0));
        cx.rel_line_to(scale!(-22.0), 0.0);
        cx.close_path();
        cx.set_source_rgb(colors::GREEN);
        cx.fill();
        cx.set_source_surface(&assets.emoticon, scale!(LEFT+12.0)+width,
                              top + scale!(TEXTBOX_HEIGHT/4.0 - 6.0));
        cx.paint();


        cx.move_to(scale!(LEFT+8.0)+width, top + scale!(TEXTBOX_HEIGHT/2.0 + 1.0));
        cx.rel_line_to(scale!(22.0), 0.0);
        cx.rel_line_to(0.0, scale!(TEXTBOX_HEIGHT/2.0 - 6.0));
        cx.arc(scale!(LEFT+25.0)+width, top+scale!(TEXTBOX_HEIGHT - 5.0), scale!(5.0),
               0.0, FRAC_PI_2);
        cx.rel_line_to(scale!(-17.0), 0.0);
        cx.close_path();
        cx.set_source_rgb(colors::GREEN);
        cx.fill();
        cx.set_source_surface(&assets.attach, scale!(LEFT+12.0)+width,
                              top + scale!(3.0*TEXTBOX_HEIGHT/4.0 - 7.0));
        cx.paint();


        cx.rounded_rectangle(self.width.get() - scale!(BIG_BUTTON_WIDTH+PADDING),
                             top, scale!(BIG_BUTTON_WIDTH), scale!(TEXTBOX_HEIGHT),
                             scale!(RADIUS));
        cx.set_source_rgb(colors::GREEN);
        cx.fill();
        cx.set_source_surface(
            &assets.sendmessage,
            self.width.get() - scale!(PADDING + BIG_BUTTON_WIDTH/2.0 + BIG_ICON_SIZE/2.0),
            top + scale!(TEXTBOX_HEIGHT/2.0 - BIG_ICON_SIZE/2.0));
        cx.paint();
    }

    pub fn draw_messages(&self, surface: &mut Surface) {
        if self.num.is_none() {
            return;
        }
        let state = self.state.borrow();
        let friend = state.friends.get(self.num.unwrap());
        let profile = &state.profile;
        let width = self.width.get() - scale!(LEFT);

        let mut surface = surface.create_for_rectangle(
            scale!(LEFT),
            scale!(HEADER_HEIGHT),
            width,
            self.height.get() - scale!(HEADER_HEIGHT + TEXTBOX_HEIGHT + 2.0 * PADDING));
        let mut cx = surface.create();

        // static NAME_WIDTH: f64 = 60f64;
        // static TIME_WIDTH: f64 = 45f64;

        let mut font = FontDescription::new();
        font.set_family("sans");
        font.set_weight(pango::WeightNormal);
        font.set_absolute_size(scale!(12.0));
        let mut layout = cx.create_pango_layout();
        layout.set_font_description(&font);

        let mut base = scale!(PADDING);
        let mut last_friend = false;
        for (i, msg) in friend.messages.iter().enumerate() {
            if i == 0 || last_friend != msg.from_friend {
                base += scale!(6.0);
                layout.set_text("");
                layout.set_alignment(AlignRight);
                layout.set_width(scale!(60.0));
                layout.set_height(-1);
                match msg.from_friend {
                    true => layout.set_text(friend.name.as_slice()),
                    false => layout.set_text(profile.name.as_slice()),
                }
                cx.move_to(0.0, base);
                cx.show_pango_layout(&layout);
                layout.set_alignment(AlignLeft);
            }
            let mut new_base = base;
            match msg.content {
                TextMessage(ref t) => {
                    layout.set_text("");
                    layout.set_height(-90);
                    layout.set_width(width - scale!(60.0 + 55.0));
                    layout.set_text(t.as_slice());
                    let (_, h) = layout.get_extends();
                    new_base += h + scale!(6.0);
                    cx.move_to(scale!(70.0), base);
                    cx.show_pango_layout(&layout);
                },
                FileMessage(..) => { },
            }
            let time = format!("{:2}:{:2}", msg.time.tm_hour, msg.time.tm_min);
            layout.set_text("");
            layout.set_width(45.0);
            layout.set_text(time.as_slice());
            cx.move_to(width - scale!(45.0), base);
            cx.show_pango_layout(&layout);

            last_friend = msg.from_friend;
            base = new_base;
        }
    }

    /*
    pub fn rel_to_textbox(&self, x: f64, y: f64) -> Option<(f64, f64)> {
        let left = scale!(LEFT+10.0+textbox::PADDING);
        let right = self.width.get() - scale!(LEFT+105.0+textbox::PADDING);
        let top = self.height.get() - scale!(TEXTBOX_HEIGHT+10.0-textbox::PADDING);
        let bottom = self.height.get() - scale!(10+textbox::PADDING);
        if x < left || x > right || y < top || y > bottom {
            None
        } else {
            Some((x - left, y - top))
        }
    }
    */

    pub fn draw_header(&self, surface: &mut Surface) {
        if self.num.is_none() {
            return;
        }
        let state = self.state.borrow();
        let assets = self.assets.borrow();
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
                cx.set_source_surface(a, scale!(PADDING), scale!(PADDING));
                cx.paint();
            },
            None => {
                let assets = self.assets.borrow();
                cx.set_source_surface(&assets.group_large_black, scale!(PADDING),
                                      scale!(PADDING));
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

        // Draw buttons
        cx.rounded_rectangle(width - scale!(2.0*(BIG_BUTTON_WIDTH+PADDING)),
                             scale!(PADDING), scale!(BIG_BUTTON_WIDTH),
                             scale!(BIG_BUTTON_HEIGHT), scale!(RADIUS));
        cx.set_source_rgb(colors::GREEN);
        cx.fill();
        cx.set_source_surface(
            &assets.call,
            width - scale!(2.0*(BIG_BUTTON_WIDTH+PADDING) - BIG_BUTTON_WIDTH/2.0 + BIG_ICON_SIZE/2.0),
            scale!(PADDING + BIG_BUTTON_HEIGHT/2.0 - BIG_ICON_SIZE/2.0));
        cx.paint();

        cx.rounded_rectangle(width - scale!(BIG_BUTTON_WIDTH+PADDING), scale!(PADDING),
                             scale!(BIG_BUTTON_WIDTH), scale!(BIG_BUTTON_HEIGHT),
                             scale!(RADIUS));
        cx.set_source_rgb(colors::GREEN);
        cx.fill();
        cx.set_source_surface(
            &assets.video,
            width - scale!(BIG_BUTTON_WIDTH+PADDING - BIG_BUTTON_WIDTH/2.0 + BIG_ICON_SIZE/2.0),
            scale!(PADDING + BIG_BUTTON_HEIGHT/2.0 - BIG_ICON_SIZE/2.0));
        cx.paint();
    }
}
