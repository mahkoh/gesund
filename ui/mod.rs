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
    pub paper: RefMut<Surface<'a>>,
    pub assets: Assets<'a>,
    pub cached: RefMut<Cached<'a>>,
    pub sidebar: sidebar::Sidebar<'a>,
    pub chatwin: chatwin::Chatwin<'a>,
    pub scale: CopyMut<f64>,
    pub width: CopyMut<f64>,
    pub height: CopyMut<f64>,
    pub should_blip: bool,
}

impl<'a> Ui<'a> {
    pub fn new(state: RefMut<State<'a>>, scale: CopyMut<f64>) -> Ui<'a> {
        let assets = Assets::new();
        let cached = Rc::new(RefCell::new(assets.cache(1.0)));
        let width = Rc::new(Cell::new(500.0f64));
        let height = Rc::new(Cell::new(500.0f64));
        let paper = image_surface_create(FormatArgb32, width.get() as i32,
                                         height.get() as i32);
        let paper = Rc::new(RefCell::new(paper));
        let sidebar = Sidebar {
            paper: paper.clone(),
            state: state.clone(),
            scroll_top: 0.0,
            scale: scale.clone(),
            height: height.clone(),
            assets: cached.clone(),
            header_dirty: true,
            controls_dirty: true,
            scroll_dirty: true,
        };
        let chatwin = Chatwin {
            state: state,
            paper: paper.clone(),
            assets: cached.clone(),
            scale: scale.clone(),
            height: height.clone(),
            width: width.clone(),
            num: None,
            header_dirty: true,
            scroll_dirty: true,
            textbox_dirty: true,
        };
        Ui {
            paper: paper,
            assets: assets,
            cached: cached,
            sidebar: sidebar,
            chatwin: chatwin,
            scale: scale,
            width: width,
            height: height,
            should_blip: true,
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

    pub fn resize(&mut self, width: int, height: int) -> bool {
        if self.width.get() as int == width && self.height.get() as int == height {
            return false;
        }
        self.width.set(width as f64);
        self.height.set(height as f64);
        let mut paper = self.paper.borrow_mut();
        *paper = image_surface_create(FormatArgb32, width as i32, height as i32);
        return true;
    }

    pub fn render(&mut self) {
        self.sidebar.render();
        self.chatwin.render();
    }

    pub fn blip(&self, dst: &mut Surface) {
        let mut cx = dst.create();
        cx.set_source_surface(&*self.paper.borrow(), 0.0, 0.0);
        cx.paint();
    }

    pub fn set_friend(&mut self, id: uint) {
        self.chatwin.num = Some(id);
    }

    pub fn all_dirty(&mut self) {
        self.sidebar.all_dirty();
        self.chatwin.all_dirty();
    }

    /*
    pub fn unset_friend(&mut self) {
        self.chatwin.num = None;
    }
    */
}
