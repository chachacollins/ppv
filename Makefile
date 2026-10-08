.PHONY: fmt build

build: main

fmt:
	rustfmt *.rs

main: main.rs raylib.rs
	rustc main.rs

