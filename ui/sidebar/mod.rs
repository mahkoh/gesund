#![feature(macro_rules)]

use std::f64::consts::{PI_2};

use utils::{RefMut, CopyMut};
use assets::{Cached};
use state::{State};
use cairo::{Surface};
use cairo::pango::{FontDescription};
use cairo::{pango, LineCapRound};
use colors;

pub static WIDTH: f64 = 222f64;
pub static HEADER_HEIGHT: f64 = 60f64;
pub static CONTROL_HEIGHT: f64 = 35f64;

macro_rules! scale {
    ($x:expr) => { self.scale.get() * $x }
}

pub struct Sidebar<'a> {
    pub state: RefMut<State<'a>>,
    pub scroll_top: f64,
    pub scale: CopyMut<f64>,
    pub height: CopyMut<f64>,
    pub assets: RefMut<Cached<'a>>,
}

impl<'a> Sidebar<'a> {
    fn draw_header(&self, surface: &mut Surface) {
        let mut cx = surface.create();
        let state = self.state.borrow();

        cx.rectangle(0.0, 0.0, scale!(WIDTH), scale!(HEADER_HEIGHT));
        cx.set_source_rgb(colors::DARK_GREY);
        cx.fill();

        let x = scale!(64.0);
        let mut y = scale!(15.0);

        cx.move_to(x, y);

        let mut font = FontDescription::new();
        font.set_family("sans");
        font.set_weight(pango::WeightBold);
        font.set_absolute_size(scale!(14.0));
        let mut layout = cx.create_pango_layout();
        layout.set_font_description(&font);
        layout.set_text(state.profile.name.as_slice());
        cx.set_source_rgb(colors::WHITE);
        cx.show_pango_layout(&layout);

        y = scale!(33.0);
        cx.move_to(x, y);
        font.set_weight(pango::WeightNormal);
        font.set_absolute_size(scale!(11.0));
        layout.set_font_description(&font);
        layout.set_text(state.profile.status.as_slice());
        cx.set_source_rgb(colors::LIGHT_GREY);
        cx.show_pango_layout(&layout);

        match state.profile.avatar {
            Some(ref a) => {
                cx.set_source_surface(a, scale!(10.0), scale!(10.0));
                cx.paint();
            },
            None => {
                let assets = self.assets.borrow();
                cx.set_source_surface(&assets.group_large, scale!(10.0), scale!(10.0));
                cx.paint();
            }
        }

        cx.set_source_rgb(colors::MEDIUM_GREY);
        cx.rounded_rectangle(scale!(WIDTH - 30.0), scale!(10.0), scale!(20.0),
                             scale!(40.0), scale!(5.0));
        cx.fill();

        cx.set_source_rgb(colors::GREEN);
        cx.arc(scale!(WIDTH - 20.0), scale!(30.0), scale!(5.0), 0.0, PI_2);
        cx.fill();
    }
    
    fn draw_controls(&self, surface: &mut Surface) {
        let mut cx = surface.create();

        let ch = scale!(CONTROL_HEIGHT);
        cx.rectangle(0.0, self.height.get() - ch, scale!(WIDTH), ch);
        cx.set_source_rgb(colors::DARK_GREY);
        cx.fill();

        let a = self.assets.borrow();
        for (i, c) in [&a.add, &a.group, &a.transfer, &a.settings].iter().enumerate() {
            let x = (i as f64 * WIDTH/4.0) + 0.5 * WIDTH/4.0 - 7.7;
            let y = CONTROL_HEIGHT/2.0 + 8.0;
            cx.set_source_surface(*c, scale!(x), self.height.get() - scale!(y));
            cx.paint();
        }
    }

    pub fn scroll(&mut self, x: int) {
        let list_height = self.list_height();
        let visible_height = self.visible_height();
        if list_height <= visible_height {
            return;
        }
        self.scroll_top -= (30.0 * x as f64) / self.list_height();
        if self.scroll_top < 0.0 {
            self.scroll_top = 0.0;
        } else if (1.0 - self.scroll_top) * list_height < visible_height {
            self.scroll_top = 1.0 - visible_height/list_height;
        }
    }

    pub fn visible_height(&self) -> f64 {
        self.height.get()/self.scale.get() - HEADER_HEIGHT - CONTROL_HEIGHT
    }

    pub fn list_height(&self) -> f64 {
        let state = self.state.borrow();
        40.0 + 50.0 * (state.groups.len() + state.friends.len()) as f64
    }

    fn draw_contacts(&self, surface: &mut Surface) {
        let state = self.state.borrow();
        let assets = self.assets.borrow();

        let height = self.height.get()/self.scale.get() - HEADER_HEIGHT - CONTROL_HEIGHT;
        if height < 0.0 {
            return;
        }

        let mut surface = surface.create_for_rectangle(0.0, scale!(HEADER_HEIGHT),
                                                       scale!(WIDTH), scale!(height));
        let mut cx = surface.create();

        cx.set_source_rgb(colors::MEDIUM_GREY);
        cx.paint();

        let mut font = FontDescription::new();
        font.set_family("sans");
        let mut layout = cx.create_pango_layout();

        let real_height = 40.0 + 50.0 * (state.groups.len() + state.friends.len()) as f64;
        let top = self.scroll_top * real_height;
        let mut base = 0.0;
        if top <= 40.0 {
            let mut x = 23.0;
            let y = 10.0 - top;
            cx.set_source_rgb(colors::DARK_GREY);
            cx.rounded_rectangle(scale!(x), scale!(y), scale!(66.0),
                                 scale!(27.0), scale!(4.0));
            cx.fill();

            font.set_weight(pango::WeightBold);
            font.set_absolute_size(scale!(10.0));
            layout.set_font_description(&font);
            layout.set_text("Online ▼");
            let (w, h) = layout.get_extends();
            cx.move_to(scale!(x + 33.0) - w/2.0, scale!(y+13.5) - h/2.0);
            cx.set_source_rgb(colors::WHITE);
            cx.show_pango_layout(&layout);

            x = 100.0;
            cx.set_source_rgb(colors::GREEN);
            cx.rounded_rectangle(scale!(x), scale!(y), scale!(115.0),
                                 scale!(27.0), scale!(4.0));
            cx.fill();

            layout.set_text("1 Friend Request");
            let (w, h) = layout.get_extends();
            cx.move_to(scale!(x + 57.5) - w/2.0, scale!(y+13.5) - h/2.0);
            cx.set_source_rgb(colors::WHITE);
            cx.show_pango_layout(&layout);
        }
        base += 40.0;

        font.set_weight(pango::WeightNormal);

        macro_rules! print_entry {
            ($name:expr, $status:expr, $icon:expr) => {
                {
                    if base + 50.0 < top {
                        base += 50.0;
                        continue;
                    }
                    if base > top + height {
                        break;
                    }

                    let x = WIDTH - 24.0;
                    let y = base - top + 25.0 - 7.5;
                    cx.set_source_surface($icon, scale!(x), scale!(y));
                    cx.paint();

                    let x = 23.5;
                    let y = base - top + 5.0;
                    cx.set_source_surface(&assets.group_large, scale!(x), scale!(y));
                    cx.paint();

                    font.set_absolute_size(scale!(12.0));
                    layout.set_font_description(&font);
                    layout.set_text($name.as_slice());
                    let (w, h) = layout.get_extends();
                    cx.move_to(scale!(73.5), scale!(y+6.0));
                    cx.set_source_rgb(colors::WHITE);
                    cx.show_pango_layout(&layout);

                    font.set_absolute_size(scale!(11.0));
                    layout.set_font_description(&font);
                    layout.set_text($status.as_slice());
                    cx.move_to(scale!(73.5), scale!(y+22.0));
                    cx.set_source_rgb(colors::LIGHT_GREY);
                    cx.show_pango_layout(&layout);

                    base += 50.0;
                }
            }
        };

        for g in state.groups.iter() {
            let icon = if g.unread {
                &assets.online_new
            } else {
                &assets.online
            };
            print_entry!(g.name, format!("{} users in chat", g.peers.len()), icon);
        }

        for f in state.friends.iter() {
            print_entry!(f.name, f.status, &assets.offline);
        }

        let bar_space = height - 24.0;
        let (bar_height, bar_top) = if height > real_height {
            (bar_space, 12.0)
        } else {
            (bar_space*height/real_height, 12.0 + self.scroll_top*bar_space)
        };
        cx.move_to(scale!(8.0), scale!(bar_top));
        cx.rel_line_to(0.0, scale!(bar_height));
        cx.set_line_width(scale!(10.0));
        cx.set_source_rgb(colors::DARK_GREY);
        cx.set_line_cap(LineCapRound);
        cx.stroke();
    }

    pub fn draw_all(&self, surface: &mut Surface) {
        self.draw_header(surface);
        self.draw_controls(surface);
        self.draw_contacts(surface);
    }
}
