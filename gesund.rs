use std;
use std::rc::{Rc};
use std::cell::{RefCell, Cell};
use std::rand::{Rng};

use time;

use state;
use state::{Profile, State, Friend, Message, TextMessage};

use ui;
use ui::{Ui};
use ui::textbox::{Textbox};

use tox::core::{Tox, UserStatusNone};

use cairo;

use sdl2;
use sdl2::video::{Window};

use bootstrap::{bootstrap};

static START_WIDTH: int = 600;
static START_HEIGHT: int = 500;

pub struct Gesund<'a> {
    // state: RefMut<State<'a>>,
    ui: Ui<'a>,
    window: Window,
    tox: Tox,
}

#[unsafe_destructor]
impl<'a> Drop for Gesund<'a> {
    fn drop(&mut self) {
        sdl2::quit();
    }
}

impl<'a> Gesund<'a> {
    pub fn new() -> Gesund {
        let scale = Rc::new(Cell::new(1.0f64));
        let state = {
            let profile = Profile {
                name:   "aグaルaーaプa".to_string(),
                status: "aグaルaーaプa".to_string(),
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
            let mut messages = Vec::new();
            let mut message = Message {
                from_friend: true,
                content: TextMessage("hello".to_string()),
                time: time::now(),
            };
            messages.push(message.clone());
            message.content = TextMessage("hurr".to_string());
            messages.push(message.clone());
            message.from_friend = false;
            message.content = TextMessage(include_str!("TEXT").to_string());
            messages.push(message.clone());
            message.from_friend = true;
            message.content = TextMessage("why are you such an asshole?".to_string());
            messages.push(message.clone());
            message.from_friend = false;
            message.content = TextMessage(";_;".to_string());
            messages.push(message.clone());
            let mut friend = Friend {
                id: 0,
                name: "mannol".to_string(),
                status: "shitposting".to_string(),
                avatar: None,
                userstatus: UserStatusNone,
                textbox: Textbox::new(ui::chatwin::TEXTBOX_HEIGHT, scale.clone()),
                messages: messages,
            };
            friend.textbox.text = (include_str!("TEXT")).to_string();
            let friends = vec!(friend);

            Rc::new(RefCell::new(State { groups: groups, profile: profile, friends: friends }))
        };
        let mut ui = Ui::new(state.clone(), scale);
        ui.resize(START_WIDTH as f64, START_HEIGHT as f64);
        ui.set_friend(0);

        sdl2::init(sdl2::InitEverything);
        let window = Window::new("gesund", sdl2::video::PosCentered,
                                 sdl2::video::PosCentered, START_WIDTH,
                                 START_HEIGHT, sdl2::video::Resizable).unwrap();

        Gesund {
            // state: state,
            ui: ui,
            window: window,
            tox: bootstrap(),
        }
    }

    pub fn run(&mut self) {
        'main : loop {
            let mut redraw = false;

            'event : loop {
                let ev = match sdl2::event::wait_event_timeout(30) {
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
                            let scale = self.ui.scale.get() + 0.1;
                            self.ui.scale(scale);
                            redraw = true;
                            break 'event;
                        }
                        if k == sdl2::keycode::CKey {
                            let scale = self.ui.scale.get() - 0.1;
                            self.ui.scale(scale);
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
                        if self.ui.sidebar.can_scroll(x as f64, y as f64) {
                            self.ui.sidebar.scroll(dx);
                            redraw = true;
                        }
                        break 'event;
                    },
                    sdl2::event::WindowEvent(_, _, sdl2::event::ExposedWindowEventId, _, _) => {
                        redraw = true;
                        break 'event;
                    }
                    _ => {},
                }
            }

            for event in self.tox.events() {
                println!("{:?}", event);
            }

            if redraw {
                {
                    let (w, h) = self.window.get_size();
                    self.ui.resize(w as f64, h as f64);
                }
                let screen = self.window.get_surface().unwrap();
                let stride = unsafe { (*screen.raw()).pitch as i32 };
                screen.with_lock(|p| {
                    let mut surface = 
                        cairo::image_surface_create_for_data(p, cairo::FormatRgb24,
                                                             self.ui.width.get() as i32,
                                                             self.ui.height.get() as i32,
                                                             stride);
                    self.ui.render();
                    self.ui.blip(&mut surface);
                });
                self.window.update_surface();
            }
        }
    }
}
