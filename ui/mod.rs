use std::rc::{Rc};
use std::cell::{RefCell, Cell};

use utils::{RefMut, CopyMut};
use state::{State};
use assets::{Assets, Cached};
use cairo::{Surface, image_surface_create, FormatArgb32};
use self::sidebar::{Sidebar};
use self::chatwin::{Chatwin};

mod sidebar;
pub mod chatwin;
pub mod textbox;

pub struct Ui<'a> {
    pub paper: Surface<'a>,

    pub assets: Assets<'a>,
    pub cached: RefMut<Cached<'a>>,
    pub sidebar: sidebar::Sidebar<'a>,
    pub chatwin: chatwin::Chatwin<'a>,
    pub scale: CopyMut<f64>,
    pub width: CopyMut<f64>,
    pub height: CopyMut<f64>,
}

impl<'a> Ui<'a> {
    pub fn new(state: RefMut<State<'a>>, scale: CopyMut<f64>) -> Ui<'a> {
        let assets = Assets::new();
        let cached = Rc::new(RefCell::new(assets.cache(1.0)));
        let width = Rc::new(Cell::new(500.0f64));
        let height = Rc::new(Cell::new(500.0f64));
        let sidebar = Sidebar {
            state: state.clone(),
            scroll_top: 0.0,
            scale: scale.clone(),
            height: height.clone(),
            assets: cached.clone(),
        };
        let chatwin = Chatwin {
            state: state,
            assets: cached.clone(),
            scale: scale.clone(),
            height: height.clone(),
            width: width.clone(),
            num: None,
        };
        Ui {
            paper: image_surface_create(FormatArgb32, width.get() as i32,
                                        height.get() as i32),

            assets: assets,
            cached: cached,
            sidebar: sidebar,
            chatwin: chatwin,
            scale: scale,
            width: width,
            height: height,
        }
    }

    pub fn scale(&mut self, mut val: f64) {
        if val < 0.1 {
            val = 0.1;
        }
        self.scale.set(val);
        let mut cached = self.cached.borrow_mut();
        *cached = self.assets.cache(val);
    }

    pub fn resize(&mut self, width: f64, height: f64) {
        self.width.set(width);
        self.height.set(height);
    }

    pub fn draw_all(&self, surface: &mut Surface) {
        self.sidebar.draw_all(surface);
        self.chatwin.draw_all(surface);
    }

    pub fn set_friend(&mut self, id: uint) {
        self.chatwin.num = Some(id);
    }

    /*
    pub fn unset_friend(&mut self) {
        self.chatwin.num = None;
    }
    */
}
