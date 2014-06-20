#![feature(macro_rules, globs)]

extern crate sdl2;
extern crate libc;
extern crate tox;
// extern crate debug;

use std::rc::{Rc};
use std::cell::{RefCell, Cell};
use std::rand::{Rng};
use state::{Profile, State, Friend};
use ui::{Ui};
use ui::textbox::{Textbox};
use assets::{Assets};
use tox::core::{UserStatusNone};

mod colors;
mod state;
mod cairo;
mod ui;
mod assets;
mod utils;

static START_WIDTH: int = 600;
static START_HEIGHT: int = 500;

fn main() {
    let scale = Rc::new(Cell::new(1.0f64));
    let mut state = {
        let profile = Profile {
            name:   "mahkoh".to_string(),
            status: "nothing".to_string(),
            avatar: None,
        };
        let mut groups = Vec::new();
        let mut rand = std::rand::task_rng();
        for i in range(0, 5) {
            let group = state::Group {
                id: 0,
                name: format!("グループ {}", i),
                peers: Vec::new(),
                unread: rand.gen(),
            };
            groups.push(group);
        }
        let mut friend = Friend {
            id: 0,
            name: "mannol".to_string(),
            status: "shitposting".to_string(),
            avatar: None,
            userstatus: UserStatusNone,
            textbox: Textbox::new(ui::chatwin::TEXTBOX_HEIGHT, scale.clone()),
        };
        friend.textbox.text = (include_str!("TEXT")).to_string();
        let friends = vec!(friend);

        Rc::new(RefCell::new(State { groups: groups, profile: profile, friends: friends }))
    };
    let mut ui = Ui::new(state.clone(), scale);
    ui.resize(START_WIDTH as f64, START_HEIGHT as f64);
    ui.set_friend(0);

    sdl2::init(sdl2::InitEverything);
    let window = sdl2::video::Window::new("gesund", sdl2::video::PosCentered,
                                          sdl2::video::PosCentered, START_WIDTH,
                                          START_HEIGHT, sdl2::video::Resizable).unwrap();

    'main : loop {
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
                    if k == sdl2::keycode::BKey {
                        let scale = ui.scale.get() + 0.1;
                        ui.scale(scale);
                        redraw = true;
                        break 'event;
                    }
                    if k == sdl2::keycode::CKey {
                        let scale = ui.scale.get() - 0.1;
                        ui.scale(scale);
                        redraw = true;
                        break 'event;
                    }
                },
                sdl2::event::WindowEvent(_, _, sdl2::event::ResizedWindowEventId, _, _) => {
                    redraw = true;
                    break 'event;
                },
                sdl2::event::MouseWheelEvent(_, _, _, dx, _) => {
                    let (_, x, y) = sdl2::mouse::get_mouse_state();
                    if ui.sidebar.can_scroll(x as f64, y as f64) {
                        ui.sidebar.scroll(dx);
                        redraw = true;
                    }
                    break 'event;
                },
                sdl2::event::WindowEvent(_, _, sdl2::event::ExposedWindowEventId, _, _) => {
                    redraw = true;
                    break 'event;
                }
                /*
                sdl2::event::MouseButtonDownEvent(_, _, _, _, x, y) => {
                    match ui.chatwin.rel_to_textbox(x, y) {
                        Some((x, y)) => {
                            let state = state.borrow();
                            let friend = state.friends.get(0);
                            friend.textbox.
                        },
                        None => { }
                    }
                }
                */
                _ => {},
            }
        }

        if redraw {
            {
                let (w, h) = window.get_size();
                ui.resize(w as f64, h as f64);
            }
            let screen = window.get_surface().unwrap();
            let stride = unsafe { (*screen.raw()).pitch as i32 };
            screen.with_lock(|p| {
                let mut surface = 
                    cairo::image_surface_create_for_data(p, cairo::FormatRgb24,
                                                         ui.width.get() as i32,
                                                         ui.height.get() as i32,
                                                         stride);
                ui.draw_all(&mut surface);
            });
            window.update_surface();
        }
    }

    sdl2::quit();
}
