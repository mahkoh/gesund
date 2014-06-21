libs = -L ../rust-sdl2/build/lib -L ../rust-tox

all:
	rustc -g $(libs) main.rs
