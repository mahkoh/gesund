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
use sdl2::keycode::{KeyCode};
use sdl2::mouse::{get_mouse_state};
use sdl2::keycode::{PlusKey, MinusKey};
use sdl2::event::{QuitEvent, KeyDownEvent, WindowEvent, MouseWheelEvent,
                  ResizedWindowEventId, ExposedWindowEventId};

use bootstrap::{bootstrap};

static START_WIDTH: int = 600;
static START_HEIGHT: int = 500;

pub struct Gesund<'a> {
    // state: RefMut<State<'a>>,
    ui: Ui<'a>,
    tox: Tox,
}

#[unsafe_destructor]
impl<'a> Drop for Gesund<'a> {
    fn drop(&mut self) {
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
        ui.resize(START_WIDTH, START_HEIGHT);
        ui.set_friend(0);

        Gesund {
            // state: state,
            ui: ui,
            tox: bootstrap(),
        }
    }

    fn handle_key(&mut self, k: KeyCode) {
        let scale = match k {
            PlusKey => self.ui.scale.get() + 0.1,
            MinusKey => self.ui.scale.get() - 0.1,
            _ => return,
        };
        self.ui.scale(scale);
        self.ui.all_dirty();
        self.ui.should_blip = true;
    }

    fn handle_resize(&mut self) {
        self.ui.all_dirty();
        self.ui.should_blip = true;
    }

    fn handle_mouse_wheel(&mut self, dx: int) {
        let (_, x, y) = get_mouse_state();
        if self.ui.sidebar.can_scroll(x as f64, y as f64) {
            self.ui.sidebar.scroll(dx);
            self.ui.sidebar.scroll_dirty = true;
            self.ui.should_blip = true;
        }
    }

    fn handle_expose(&mut self) {
        self.ui.should_blip = true;
    }

    pub fn run(&mut self) {
        sdl2::init(sdl2::InitEverything);

        let window = Window::new("gesund", sdl2::video::PosCentered,
                                 sdl2::video::PosCentered, START_WIDTH,
                                 START_HEIGHT, sdl2::video::Resizable).unwrap();

        'main : loop {
            'event : loop {
                let ev = match sdl2::event::wait_event_timeout(30) {
                    Ok(e) => e,
                    Err(..) => break 'event,
                };
                match ev {
                    QuitEvent(_) => break 'main,
                    KeyDownEvent(_, _, k, _, _) => self.handle_key(k),
                    WindowEvent(_, _, ResizedWindowEventId, _, _) => self.handle_resize(),
                    WindowEvent(_, _, ExposedWindowEventId, _, _) => self.handle_expose(),
                    MouseWheelEvent(_, _, _, dx, _) => self.handle_mouse_wheel(dx),
                    _ => continue 'event,
                }
                break 'event;
            }

            /*
            for event in self.tox.events() {
                println!("{:?}", event);
            }
            */

            if self.ui.should_blip {
                let (w, h) = window.get_size();
                if self.ui.resize(w, h) {
                    self.ui.all_dirty();
                }
                let screen = window.get_surface().unwrap();
                let stride = unsafe { (*screen.raw()).pitch as i32 };
                screen.with_lock(|p| {
                    let mut surface = 
                        cairo::image_surface_create_for_data(p, cairo::FormatRgb24,
                                                             w as i32, h as i32, stride);
                    self.ui.render();
                    self.ui.blip(&mut surface);
                    self.ui.should_blip = false;
                });
                window.update_surface();
            }
        }

        sdl2::quit();
    }
}
