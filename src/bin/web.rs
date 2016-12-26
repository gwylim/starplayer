#![feature(link_args)]
extern crate starplayer;

use starplayer::StarAI;
use std::mem::transmute;

const KOMI: isize = 1;

#[link_args = "-s EXPORTED_FUNCTIONS=['_main','_starplayer_new','_starplayer_calculate','_starplayer_best_move','_starplayer_add_move','_starplayer_destroy'] -s NO_EXIT_RUNTIME=1 -s TOTAL_MEMORY=1073741824"]
extern {}

#[no_mangle]
pub extern fn starplayer_new(size: i32) ->  *mut StarAI {
    unsafe { transmute(Box::new(StarAI::new(size as usize))) }
}

#[no_mangle]
pub extern fn starplayer_calculate(ptr: *mut StarAI, iterations: i32) {
    let mut star = unsafe { &mut *ptr };
    star.calculate(iterations as usize, KOMI);
}

// Move is returned as x + y * (size + size - 1), since complex return types are hard
#[no_mangle]
pub extern fn starplayer_best_move(ptr: *mut StarAI) -> i32 {
    let star = unsafe { &*ptr };
    let (x, y) = star.best_move();
    let size = star.size();
    (x + y * (size + size - 1)) as i32
}

#[no_mangle]
pub extern fn starplayer_add_move(ptr: *mut StarAI, x: i32, y: i32) {
    let mut star = unsafe { &mut *ptr };
    star.add_move(x as usize, y as usize);
}

#[no_mangle]
pub extern fn starplayer_destroy(ptr: *mut StarAI) {
    unsafe { transmute::<*mut StarAI, Box<StarAI>>(ptr) };
}

fn main() {
}
