#![feature(macro_rules, globs)]

extern crate sdl2;
extern crate libc;
extern crate tox;
extern crate time;
extern crate debug;

mod colors;
mod state;
mod cairo;
mod ui;
mod assets;
mod utils;
mod gesund;
mod bootstrap;

fn main() {
    gesund::Gesund::new().run();
}
