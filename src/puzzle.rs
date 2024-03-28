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
    // TODO: Implement image textures as parts
    textures: Option<Vec<Texture2D>>,
    draw_image_mode: bool,
    images: Vec<Image>,
    image_count: i32,
    image_selection: i32,
}

impl Puzzle {

    pub async fn new(img_path: &str) -> Self {
        let mut tiles: [i32; 9] = (0..9).collect::<Vec<i32>>().try_into().unwrap();
        tiles[8] = -1;
        let dimension = Vec2::new(600.0, 600.0);
        let mut image_names: Vec<String> = Vec::new();
        let mut image_count = 0;
        if let Ok(entries) = fs::read_dir(img_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.path().extension().and_then(OsStr::to_str) == Some("png") {
                        image_names.push(entry.path().into_os_string().into_string().expect("invalid filename"));
                        image_count += 1;
                    }
                }
            }
        }
        image_names.sort();
        let images: Vec<Image> = join_all(image_names.iter().map(|path| async { load_image(path.as_str()).await.expect("Failure to load image") } )).await;
        Self {
            position: Vec2::new(100., 100.), 
            dimension, 
            tile_size: dimension.x / 3.0, 
            tiles, 
            textures: None, 
            draw_image_mode: false, 
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

    fn draw_numbered_tile(&self, tile: i32, x_pos: f32, y_pos: f32, color: Color, disp_number: bool) {
        // Draws numbered tiles as text
        draw_rectangle(x_pos, y_pos, self.tile_size, self.tile_size, color);

        if disp_number {
            let size = 100;
            let txt = tile.to_string();
            let font_center = get_text_center(txt.as_str(), Option::None, size, 1.0, 0.0);
            draw_text(txt.as_str(), x_pos + self.tile_size/2. - font_center.x, y_pos + self.tile_size/2. - font_center.y, size as f32, WHITE);
        }

        draw_rectangle_lines(x_pos, y_pos, self.tile_size, self.tile_size, 5., BLACK);
    }

    fn get_tile_position(&self, tile_pos: usize) -> Vec2 {
        Vec2::new(
            self.position.x + (tile_pos % 3) as f32 * self.tile_size,
            self.position.y + (self.tile_size * (tile_pos / 3) as f32)
        )
    }   

    fn check_mouse_intersections(&self) -> Option<i32> {
        // Return tile that is currently selected by mouse

        let mouse_pos = mouse_position();
        for (i, tile) in self.tiles.into_iter().enumerate() {
            let tile_pos = self.get_tile_position(i);
            if mouse_pos.0 >= tile_pos.x && mouse_pos.0 <= tile_pos.x + self.tile_size {
                if mouse_pos.1 >= tile_pos.y && mouse_pos.1 <= tile_pos.y + self.tile_size {
                    // Draw transparent rect for now
                    println!("Mouse over tile {i}");
                    return Some(i as i32);
                }
            }
        }
        None
    }

    pub fn draw(&self) {
        let GLASS_BLUE = Color::new(0., 0., 1., 0.5);
        draw_text(self.image_selection.to_string().as_str(), 10.0, 30.0, 30.0, WHITE);
        //draw_texture_ex(&self.texture, self.position.x, self.position.y, WHITE, DrawTextureParams {dest_size: Some(self.dimension), source: None, rotation: 0., flip_x: false, flip_y: false, pivot: None});
        //draw_rectangle_lines(49.5, 49.25, 500.5, 500.5, 5.0, BLACK);


        for (i, tile) in self.tiles.into_iter().enumerate() {
            let tile_pos = self.get_tile_position(i);
            if self.draw_image_mode == true {
                if let Some(textures) = self.textures.as_ref() {
                    if tile != -1 {
                        draw_texture(&textures[tile as usize], tile_pos.x, tile_pos.y, WHITE);		
                    }
                } else {
                    self.draw_numbered_tile(tile, tile_pos.x, tile_pos.y, GOLD, true);
                }
            } else {
                    self.draw_numbered_tile(tile, tile_pos.x, tile_pos.y, GOLD, true);
            }
        }

        let selected_tile = self.check_mouse_intersections();

        if let Some(tile) = selected_tile {
            let pos = self.get_tile_position(tile as usize);
            self.draw_numbered_tile(tile, pos.x, pos.y, GLASS_BLUE, false);
        }
    }

    pub fn update(&mut self) {
        match self.textures {
            None => {
                println!("Loaded texture!");
                self.load_texture()
            }
            _ => {},
        }

        if is_key_pressed(KeyCode::Right) {
            self.image_selection += 1;
            if self.image_selection >= self.image_count {
                self.image_selection = 0;
            }
            println!("Changed image!");
            self.load_texture();
        }

        if is_key_pressed(KeyCode::Left) {
            self.image_selection -= 1;
            if self.image_selection < 0 {
                self.image_selection = self.image_count - 1;
            }
            println!("Changed image!");
            self.load_texture();
        }

        if is_key_pressed(KeyCode::Space) {
            self.draw_image_mode = !self.draw_image_mode;
        }

        self.check_mouse_intersections();
    }
}

