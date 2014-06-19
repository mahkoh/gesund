libs = -L ../rust-sdl2/build/lib -L ../rust-tox

all:
	rustc $(libs) test.rs
