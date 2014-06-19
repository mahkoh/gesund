#![feature(macro_rules, globs)]
#![allow(attribute_usage, unused_attribute)]

extern crate libc;
extern crate sdl2;

use kit::{Button, Drawable, Container};
use cairo::{Surface, RGB, svg};
use cairo::pango::{FontDescription, Layout};
use cairo::pango;

mod cairo;
mod kit;

static item_width: f64 = 200.0;
static item_height: f64 = 49.0;
static main_x: f64 = 10.0 + item_width + 5.0 + 10.0 + 10.0 + 1.0;
static edit_height: f64 = 100.0;

fn main() {
    CairoTox { WIDTH: 600.0, HEIGHT: 500.0 }.main();
}

struct CairoTox {
    WIDTH: f64,
    HEIGHT: f64,
}

impl CairoTox {
    fn draw_background(&self, surface: &mut Surface) {
        let mut cx = surface.create();
        cx.set_line_width(1.0);

        cx.set_source_rgb(RGB(1.0, 1.0, 1.0));
        cx.paint();

        let bc = 0x99 as f64/0xff as f64;
        cx.set_source_rgb(RGB(bc, bc, bc));
        cx.rectangle(0.5, 0.5, self.WIDTH-1.0, self.HEIGHT-1.0);
        cx.stroke();

        let inner_border = RGB(167.0/255.0, 215.0/255.0, 249.0/255.0);
        cx.set_source_rgb(inner_border);
        cx.set_line_width(1.0);

        cx.move_to(10.0, self.HEIGHT-10.5);
        cx.rel_line_to(item_width + 5.5, 0.0);
        cx.rel_line_to(0.0, -self.HEIGHT + 20.5);
        cx.stroke();

        cx.move_to(self.WIDTH-10.0, self.HEIGHT-10.5);
        cx.rel_line_to(main_x-self.WIDTH-0.5, 0.0);
        cx.rel_line_to(0.0, -self.HEIGHT+30.5);
        cx.stroke();
    }

    fn draw_sysbar(&self, surface: &mut Surface) {
        let underline = svg::Handle::from_slice(include_bin!("svgs/ul.svg")).unwrap();
        let win       = svg::Handle::from_slice(include_bin!("svgs/win.svg")).unwrap();
        let x         = svg::Handle::from_slice(include_bin!("svgs/x.svg")).unwrap();

        for (i, p) in [underline, win, x].iter().enumerate() {
            let x = self.WIDTH - 60.0 + (20.0 * i as f64);
            let mut sub = surface.create_for_rectangle(x, 10.0, 10.0, 10.0);
            p.render_cairo(sub.create());
        }
    }

    fn draw_friends(&self, surface: &mut Surface) {
        let mahkoh = Friend {
            name: "mahkoh-chan".to_string(),
            status: "Shitposting".to_string(),
            online: true,
        };
        let stqism = Friend {
            name: "stqism-kun".to_string(),
            status: "シリング".to_string(),
            online: false,
        };
        let astonex = Friend {
            name: "astonex".to_string(),
            status: "watching shit anime".to_string(),
            online: true,
        };
        for (i, p) in [mahkoh, stqism, astonex].iter().enumerate() {
            let y = 10.0 + (i as f64 * item_height);
            let mut sub = surface.create_for_rectangle(10.0, y, self.WIDTH, self.HEIGHT);
            p.draw_to(&mut sub);
        }
    }

    fn draw_edit(&self, surface: &mut Surface) {
        {
            let mut cx = surface.create();
            cx.set_line_width(1.0);
            cx.set_source_rgb_hex(0xcccccc);
            cx.rectangle(main_x-0.5, self.HEIGHT-20.5-edit_height, self.WIDTH-10.0-main_x, edit_height);
            cx.stroke();
        }

        let mut sub = surface.create_for_rectangle(main_x+5.0, self.HEIGHT-15.0-edit_height,
                                               self.WIDTH-20.0-main_x, edit_height - 11.0);
        let mut cx = sub.create();

        let text = "What the fuck did you just fucking say about me, you little bitch? I’ll have you know I graduated top of my class in the Navy Seals, and I’ve been involved in numerous secret raids on Al-Quaeda, and I have over 300 confirmed kills. I am trained in gorilla warfare and I’m the top sniper in the entire US armed forces. You are nothing to me but just another target. I will wipe you the fuck out with precision the likes of which has never been seen before on this Earth, mark my fucking words. You think you can get away with saying that shit to me over the Internet? Think again, fucker. As we speak I am contacting my secret network of spies across the USA and your IP is being traced right now so you better prepare for the storm, maggot. The storm that wipes out the pathetic little thing you call your life. You’re fucking dead, kid. I can be anywhere, anytime, and I can kill you in over seven hundred ways, and that’s just with my bare hands. Not only am I extensively trained in unarmed combat, but I have access to the entire arsenal of the United States Marine Corps and I will use it to its full extent to wipe your miserable ass off the face of the continent, you little shit. If only you could have known what unholy retribution your little “clever” comment was about to bring down upon you, maybe you would have held your fucking tongue. But you couldn’t, you didn’t, and now you’re paying the price, you goddamn idiot. I will shit fury all over you and you will drown in it. You’re fucking dead, kiddo.";

        let mut font = FontDescription::new();
        font.set_family("sans");
        font.set_weight(pango::WeightNormal);
        font.set_absolute_size(12.0);
        let mut layout = cx.create_pango_layout();
        layout.set_font_description(&font);
        layout.set_text(text);
        layout.set_width(self.WIDTH-20.0-main_x);
        cx.set_source_rgb_hex(0x000000);
        cx.show_pango_layout(&layout);
    }

    fn main(&mut self) {
        sdl2::init(sdl2::InitEverything);
        let mut window = sdl2::video::Window::new("cairotox", sdl2::video::PosCentered,
                                              sdl2::video::PosCentered, self.WIDTH as int,
                                              self.HEIGHT as int,
                                              sdl2::video::Resizable).unwrap();;
        let mut screen = window.get_surface().unwrap();;

        let stride = unsafe { (*screen.raw()).pitch as i32 };
        screen.with_lock(|p| {
            let mut surface = 
                cairo::image_surface_create_for_data(p, cairo::FormatRgb24,
                                                     self.WIDTH as i32,
                                                     self.HEIGHT as i32,
                                                     stride);
            self.draw_all(&mut surface);
        });
        window.update_surface();


        'main : loop {
            let mut resize = None;
            'event : loop {
                let ev = match sdl2::event::wait_event() {
                    Ok(e) => e,
                    Err(..) => break 'event,
                };
                match ev {
                    sdl2::event::QuitEvent(_) => break 'main,
                    sdl2::event::NoEvent => break 'event,
                    sdl2::event::KeyDownEvent(_, _, k, _, _) => {
                        if k == sdl2::keycode::AKey {
                            break 'main;
                        }
                    },
                    sdl2::event::WindowEvent(_, _, sdl2::event::ResizedWindowEventId, w, h) => {
                        resize = Some((w, h));
                        break 'event;
                    }
                    _ => {}
                }
            }

            match resize {
                Some((w, h)) => {
                    self.WIDTH = w as f64;
                    self.HEIGHT = h as f64;
                    screen = window.get_surface().unwrap();
                    let stride = unsafe { (*screen.raw()).pitch as i32 };
                    screen.with_lock(|p| {
                        let mut surface = 
                            cairo::image_surface_create_for_data(p, cairo::FormatRgb24,
                                                                 w as i32,
                                                                 h as i32,
                                                                 stride);
                        self.draw_all(&mut surface);
                    });
                    assert!(window.update_surface());
                },
                None => { }
            }
        }

        sdl2::quit();
    }

    fn draw_all(&self, surface: &mut Surface) {
        self.draw_background(surface);
        self.draw_sysbar(surface);
        self.draw_friends(surface);
        self.draw_edit(surface);
    }
}

struct Friend {
    name: String,
    status: String,
    online: bool,
}

impl Drawable for Friend {
    fn draw_to(&self, surface: &mut Surface) {
        {
            static head_svg: &'static [u8] = include_bin!("svgs/cli/group.svg");
            let head = svg::Handle::from_slice(head_svg).unwrap();
            let (width, height) = head.dimensions();
            let mut sub = surface.create_for_rectangle(11.0, 11.0, width+1.0, height+1.0);
            head.render_cairo(sub.create());
        }

        let mut cx = surface.create();

        match self.online {
            true => cx.set_source_rgb_hex(0x00aa00),
            false => cx.set_source_rgb_hex(0xaa0000),
        }
        cx.arc(8.0, 8.0, 5.0, 0.0, std::f64::consts::PI_2);
        cx.fill();

        let mut font = FontDescription::new();
        font.set_family("sans");
        font.set_weight(pango::WeightNormal);
        font.set_absolute_size(15.0);
        {
            cx.set_source_rgb_hex(0x333333);
            let mut layout = cx.create_pango_layout();
            layout.set_font_description(&font);
            layout.set_text(self.name.as_slice());
            cx.move_to(50.0, 6.0);
            cx.show_pango_layout(&layout);
        }
        font.set_absolute_size(13.0);
        {
            cx.set_source_rgb_hex(0x999999);
            let mut layout = cx.create_pango_layout();
            layout.set_font_description(&font);
            layout.set_text(self.status.as_slice());
            cx.move_to(50.0, 25.0);
            cx.show_pango_layout(&layout);
        }

        cx.set_line_width(1.0);
        cx.set_source_rgb_hex(0xeeeeee);
        cx.move_to(0.0, item_height - 0.5); 
        cx.rel_line_to(item_width, 0.0);
        cx.stroke();
    }

    fn width(&self) -> f64 {
        200.0
    }

    fn height(&self) -> f64 {
        49.0
    }
}
