use std::fs;
use std::ffi::OsStr;
use std::convert::TryInto;
use macroquad::prelude::*;

pub mod puzzle;



#[macroquad::main("BasicShapes")]
async fn main() {
    request_new_screen_size(700., 700.);


    println!("{:?}", std::env::current_dir().unwrap());
    let img_path = if cfg!(windows) {
        r"..\img\"

    } else {
        r"img/"
    };
    let mut puzzle = Puzzle::new();
	//puzzle.load_texture("").await;
    let mut images: Vec<String> = Vec::new(); 
	if let Ok(entries) = fs::read_dir(img_path) {
		for entry in entries {
			if let Ok(entry) = entry {
				let filename = entry.file_name();
				if entry.path().extension().and_then(std::ffi::OsStr::to_str) == Some("png") {
					if let Some(path_str) = filename.to_str() {
						images.push(path_str.to_string());
					}
				}
			}
		}
	}
    println!("{:?}", images);
    println!("{:?}", puzzle.tiles);

    /*
    loop {
        clear_background(GRAY);
        if is_key_down(KeyCode::Escape) {
            break;
        }        
		puzzle.draw();
        next_frame().await
    }
    */

}
