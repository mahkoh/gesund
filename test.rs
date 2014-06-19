#![feature(macro_rules, globs)]

extern crate sdl2;
extern crate libc;
extern crate tox;

use state::{Profile};
use sidebar::{Sidebar};
use std::rc::{Rc};
use std::cell::{RefCell};
use std::rand::{Rng};
use assets::{Assets};

mod colors;
mod state;
mod cairo;
mod sidebar;
mod assets;

static START_WIDTH: int = 600;
static START_HEIGHT: int = 500;

fn main() {
    let mut width = START_WIDTH;
    let mut height = START_HEIGHT;
    let scale = 2.0;

    let profile = Profile {
        name:   "mahkoh".to_string(),
        status: "nothing".to_string(),
        avatar: None,
    };
    let mut groupchats = Vec::new();
    let mut rand = std::rand::task_rng();
    for i in range(0, 22) {
        let group = state::Group {
            id: 0,
            name: format!("group {}", i),
            peers: Vec::new(),
            unread: rand.gen(),
        };
        groupchats.push(group);
    }
    let friends = Vec::new();

    let mut sidebar = Sidebar {
        profile: Rc::new(RefCell::new(profile)),
        groups:  Rc::new(RefCell::new(groupchats)),
        friends: Rc::new(RefCell::new(friends)),
        scroll_top: 0.0,
        scale: scale,
        height: START_HEIGHT as f64,
        assets: Rc::new(RefCell::new(Assets::new().cache(scale))),
    };


    sdl2::init(sdl2::InitEverything);
    let mut window = sdl2::video::Window::new("gesund", sdl2::video::PosCentered,
                                          sdl2::video::PosCentered, width, height,
                                          sdl2::video::Resizable).unwrap();;
    let mut screen = window.get_surface().unwrap();;

    let stride = unsafe { (*screen.raw()).pitch as i32 };
    screen.with_lock(|p| {
        let mut surface = 
            cairo::image_surface_create_for_data(p, cairo::FormatRgb24,
                                                 width as i32,
                                                 height as i32,
                                                 stride);
        sidebar.draw_all(&mut surface);
    });
    window.update_surface();


    'main : loop {
        let mut resize = None;
        let mut redraw = false;

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
                },
                sdl2::event::MouseWheelEvent(_, _, _, x, _) => {
                    sidebar.scroll(x);
                    redraw = true;
                    break 'event;
                },
                sdl2::event::WindowEvent(..) => {
                    redraw = true;
                }
                _ => {},
            }
        }

        match resize {
            Some((w, h)) => {
                width = w;
                height = h;
                sidebar.height = h as f64;
                screen = window.get_surface().unwrap();
                redraw = true;
            },
            None => { }
        }

        if redraw {
            let stride = unsafe { (*screen.raw()).pitch as i32 };
            screen.with_lock(|p| {
                let mut surface = 
                    cairo::image_surface_create_for_data(p, cairo::FormatRgb24,
                                                         width as i32,
                                                         height as i32,
                                                         stride);
                sidebar.draw_all(&mut surface);
            });
            assert!(window.update_surface());
        }
    }

    sdl2::quit();
}
