//Run with  `rustc +stage1 -Z codegen-backend=cranelift --out-dir build cranelift-test.rs`

#![feature(no_core)]
#![feature(lang_items)]

// Look at me.
// Look at me.
// I'm the libcore now.
#![no_core]

// Tell the compiler to link to appropriate runtime libs
// (This way I don't have to specify `-l` flags explicitly)
#[cfg(target_os = "linux")]
#[link(name = "c")]
extern {}
#[cfg(target_os = "macos")]
#[link(name = "System")]
extern {}

// Compiler needs these to proceed
#[lang = "sized"]
pub trait Sized {}
#[lang = "copy"]
pub trait Copy {}
#[lang = "freeze"]
trait Freeze {}

#[lang = "add"]
trait MyAdd<RHS> {
    type Output;
    fn add(self, other: RHS) -> Self::Output;
}

impl MyAdd<isize> for isize {
    type Output = isize;
    fn add(self, other: isize) -> isize {
        self + other
    }
}


// `main` isn't the actual entry point, `start` is.
#[lang = "start"]
fn start(_main: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
    // we can't really do much in this benighted hellhole of
    // an environment without bringing in more libraries.
    // We can make syscalls, segfault, and set the exit code.
    // To be sure that this actually ran, let's set the exit code.
    let x = 4;
    x + 2
}

// still need a main unless we want to use `#![no_main]`
// won't actually get called; `start()` is supposed to call it
fn main() {}
