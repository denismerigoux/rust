#![feature(no_core)]
#![no_core]
#![feature(lang_items)]

#[lang="sized"]
trait Sized {}

#[lang = "copy"]
trait Copy {}

#[lang = "freeze"]
trait Freeze {}


#[lang = "add"]
trait MyAdd<RHS> {
    type Output;
    fn add(self, other: RHS) -> Self::Output;
}


impl MyAdd<u32> for u32 {
    type Output = u32;
    fn add(self, other: u32) -> u32 {
        self + other
    }
}


fn main() {
    let x : u32 = 1;
    let _y : u32 = x + 2;
}
