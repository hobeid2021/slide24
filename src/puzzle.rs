use std::ffi::OsStr;
use std::fs;
use macroquad::prelude::*;
use futures::future::join_all;


#[derive(Debug)]
pub struct Puzzle {
    position: Vec2, // From top left corner
    dimension: Vec2,	
    tile_size: f32,
    tiles: [i32; 9],
    selected_tile: Option<usize>,
    // TODO: Implement image textures as parts
    textures: Option<Vec<Texture2D>>,
    draw_image_mode: bool,
    images: Vec<Image>,
    image_count: i32,
    image_selection: i32,
}

static GLASS_BLUE: macroquad::color::Color = Color
    {r: 0., g: 0., b: 1., a: 0.5};

impl Puzzle {

    pub async fn new(img_path: &str) -> Self {
        let mut tiles: [i32; 9] = (0..9).collect::<Vec<i32>>().try_into().unwrap();
        tiles[8] = -1; // Negative tile represents empty space
        let dimension = Vec2::new(600.0, 600.0);
        
        let mut image_names: Vec<_> = fs::read_dir(img_path).expect("Invalid image path")
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().and_then(OsStr::to_str) == Some("png"))
            .filter_map(|entry| Some(entry.path().into_os_string().into_string().expect("Invalid filename")))
            .collect::<Vec<String>>();

        image_names.sort();
        let image_count = image_names.len() as i32;

        // Load images in alphanumerical order (01, 02, 03) instead of just randomly
        let images: Vec<Image> = join_all(image_names.iter().map(|path| async { load_image(path.as_str()).await.expect("Failure to load image") } )).await;
        Self {
            position: Vec2::new(100., 100.), 
            dimension, 
            tile_size: dimension.x / 3.0, 
            tiles, 
            selected_tile: None,
            textures: None, 
            draw_image_mode: true, 
            images, 
            image_count,
            image_selection: 0
        }

    }


    pub fn load_texture(&mut self) {
        let puzzle_image = &self.images[self.image_selection as usize]; 
        // Take slices of texture image
        let sub_images = self.tiles.into_iter().map(|n| Texture2D::from_image(&puzzle_image.sub_image(Rect {x: (n % 3) as f32 * self.tile_size, y: self.tile_size * (n / 3) as f32, w: self.tile_size, h: self.tile_size} )))
            .collect::<Vec<Texture2D>>();
        self.textures = Some(sub_images);
    }

    fn draw_tile(&self, tile: i32, x_pos: f32, y_pos: f32, color: Color, fill: bool, disp_number: bool) {
        // Draws numbered tiles as text
        if fill { 
            draw_rectangle(x_pos, y_pos, self.tile_size, self.tile_size, color);
        } else {
            draw_rectangle_lines(x_pos, y_pos, self.tile_size, self.tile_size, 5., color);
        }

        if disp_number {
            let size = 100;
            let txt = tile.to_string();
            let font_center = get_text_center(txt.as_str(), Option::None, size, 1.0, 0.0);
            draw_text(txt.as_str(), x_pos + self.tile_size/2. - font_center.x, y_pos + self.tile_size/2. - font_center.y, size as f32, WHITE);
        }

    }

    fn get_tile_position(&self, tile_pos: usize) -> Vec2 {
        Vec2::new(
            self.position.x + (tile_pos % 3) as f32 * self.tile_size,
            self.position.y + (self.tile_size * (tile_pos / 3) as f32)
        )
    }   

    fn check_mouse_intersections(&self) -> Option<usize> {
        // Return tile that is currently selected by mouse
        let mouse_pos = mouse_position();
        for (i, tile) in self.tiles.into_iter().enumerate() {
            let tile_pos = self.get_tile_position(i);
            if mouse_pos.0 >= tile_pos.x && mouse_pos.0 <= tile_pos.x + self.tile_size {
                if mouse_pos.1 >= tile_pos.y && mouse_pos.1 <= tile_pos.y + self.tile_size {
                    // Draw transparent rect for now
                    return Some(i);
                }
            }
        }
        None
    }

    pub fn draw(&self) {
        draw_text(self.image_selection.to_string().as_str(), 10.0, 30.0, 30.0, WHITE);

        for (i, tile) in self.tiles.into_iter().enumerate() {
            let tile_pos = self.get_tile_position(i);
            if self.draw_image_mode == true {
                if let Some(textures) = self.textures.as_ref() {
                    if tile != -1 {
                        draw_texture(&textures[tile as usize], tile_pos.x, tile_pos.y, WHITE);		
                    }
                } else {
                    self.draw_tile(tile, tile_pos.x, tile_pos.y, GOLD, true, true);
                }
            } else {
                    self.draw_tile(tile, tile_pos.x, tile_pos.y, GOLD, true, true);
            }
        }

        if let Some(tile) = self.check_mouse_intersections() {
            // Highlight tile under mouse
            let pos = self.get_tile_position(tile as usize);
            self.draw_tile(self.tiles[tile], pos.x, pos.y, GLASS_BLUE, false, false);
        }

        if let Some(tile) = self.selected_tile {
            let pos = self.get_tile_position(tile as usize);
            self.draw_tile(self.tiles[tile], pos.x, pos.y, GREEN, false, false);

        }

    }

    pub fn update(&mut self) {
        match self.textures {
            None => {
                self.load_texture()
            }
            _ => {},
        }

        if is_key_pressed(KeyCode::Right) {
            self.image_selection += 1;
            if self.image_selection >= self.image_count {
                self.image_selection = 0;
            }
            self.load_texture();
        }

        if is_key_pressed(KeyCode::Left) {
            self.image_selection -= 1;
            if self.image_selection < 0 {
                self.image_selection = self.image_count - 1;
            }
            self.load_texture();
        }

        if is_key_pressed(KeyCode::Space) {
            self.draw_image_mode = !self.draw_image_mode;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
             if let Some(pressed_tile) = self.check_mouse_intersections() {

                 // If tile is already selected swap
                 // TODO: Check if swap between selected and empty is valid
                 if let Some(already_selected) = self.selected_tile {
                     if self.tiles[pressed_tile] == -1 {
                        /*
                         *
                         *   Represent sliding puzzle as part of a larger grid to check move
                         *   vaildity (from player selected tile to empty tile -1)
                         *
                         *   0  1  2  3  4
                         *   5  6  7  8  9
                         *  10 11 12 13 14
                         *  15 16 17 18 19
                         *  20 21 22 23 24
                         *
                         *
                         */

                        // Validate move
                        let pressed_tile_offset = pressed_tile + 5 * (1 + pressed_tile / 3);
                        let already_selected_offset = already_selected + 5 * (1 + pressed_tile / 3);
                        
                        // Horizontal
                        let horizontal_slide = if pressed_tile_offset == already_selected_offset - 1 || pressed_tile_offset == already_selected_offset + 1 {
                            true
                        } else {
                            false
                        };

                        

                        // Vertical
                        let vertical_slide = if pressed_tile_offset == already_selected_offset - 3 || pressed_tile_offset == already_selected_offset + 3 {
                            true
                        } else {
                            false
                        };

                        if horizontal_slide || vertical_slide {
                            self.tiles.swap(pressed_tile, already_selected);
                        }
                     }

                     self.selected_tile = None;

                 } else {

                    self.selected_tile = Some(pressed_tile);

                 }
             } else {
                 self.selected_tile = None;
             }
        } 
       
    }
}

