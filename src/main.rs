// By: Hadi Obeid 
// Email: hobeid1212@gmail.com
use std::fs;
use std::ffi::OsStr;
use macroquad::prelude::*;

pub mod animation;
pub mod puzzle;

use crate::puzzle::Puzzle;

#[macroquad::main("Sliding Puzzle")]
async fn main() {
    request_new_screen_size(800., 800.);


    println!("{:?}", std::env::current_dir().unwrap());
    // Run from root directory slide24/
    let img_path = if cfg!(windows) {
        r".\img\"
    } else {
        r"./img/"
    };

    let mut puzzle = Puzzle::new(img_path).await;

    loop {
        clear_background(GRAY);
        if is_key_down(KeyCode::Escape) {
            break;
        }        
        puzzle.update();
		puzzle.draw();
        next_frame().await
    }

}
