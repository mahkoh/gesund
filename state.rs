use tox::core::{UserStatus};
use cairo::{Surface};
use ui::textbox::{Textbox};
use time::{Tm};
use utils::{RefMut};

pub struct Peer {
    pub id: i32,
    pub name: String,
}

pub struct Group {
    pub id: i32,
    pub name: String,
    pub peers: Vec<Peer>,
    pub unread: bool,
}

pub struct Profile<'a> {
    pub name: String,
    pub status: String,
    pub avatar: Option<Surface<'a>>,
}

pub struct Friend<'a> {
    pub id: i32,
    pub name: String,
    pub status: String,
    pub avatar: Option<Surface<'a>>,
    pub userstatus: UserStatus,
    pub textbox: Textbox,
    // pub messages: Vec<Message>,
}

pub struct Message {
    pub from_friend: bool,
    pub text: String,
    pub time: Tm,
}

pub enum MessageType {
    TextMessage(String),
    FileMessage(RefMut<FileTransfer>),
}

pub struct FileTransfer {
    pub name: String,
}

pub struct State<'a> {
    pub groups: Vec<Group>,
    pub profile: Profile<'a>,
    pub friends: Vec<Friend<'a>>,
}
