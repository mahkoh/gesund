use tox::core::{UserStatus};
use cairo::{Surface};
use ui::textbox::{Textbox};

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
    //pub messages: Vec<Message>,
}

pub struct State<'a> {
    pub groups: Vec<Group>,
    pub profile: Profile<'a>,
    pub friends: Vec<Friend<'a>>,
}
