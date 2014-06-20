libs = -L ../rust-sdl2/build/lib -L ../rust-tox

all:
	rustc -O $(libs) test.rs
