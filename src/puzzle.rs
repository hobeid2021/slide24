use std::ffi::OsStr;
use std::fs;
use macroquad::prelude::*;

#[derive(Debug)]
pub struct Puzzle {
    position: Vec2, // From top left corner
    dimension: Vec2,	
    tile_size: f32,
    tiles: [i32; 9],
    // TODO: Implement image textures as parts
    textures: Option<Vec<Texture2D>>,
    draw_image_mode: bool,
    images: Option<Vec<Image>>,
    image_selection: usize,
}

impl Puzzle {

    pub async fn new(img_path: &str) -> Self {
        let tiles: [i32; 9] = (0..9).collect::<Vec<i32>>().try_into().unwrap();
        let dimension = Vec2::new(600.0, 600.0);
        let mut images: Vec<Image> = Vec::new(); 

        if let Ok(entries) = fs::read_dir(img_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.path().extension().and_then(OsStr::to_str) == Some("png") {
                        images.push(load_image(entry.path().into_os_string().into_string().expect("invalid filename").as_str()).await.expect("Failure to load image"));
                    }
                }
            }
        }

        Self {
            position: Vec2::new(50.,50.), 
            dimension, 
            tile_size: dimension.x / 3.0, 
            tiles, 
            textures: None, 
            draw_image_mode: true, 
            images: Some(images), 
            image_selection: 0
        }

    }

    pub fn init_texture(&mut self) {
        // If textures are present init them
        if let Some(_images) = self.images.as_ref() {
            self.load_texture();
        }
    }

    pub fn load_texture(&mut self) {
        if let Some(images) = self.images.as_ref() {
            let puzzle_image = &images[self.image_selection]; 
            // Take slices of texture image
            let sub_images = self.tiles.into_iter().map(|n| Texture2D::from_image(&puzzle_image.sub_image(Rect {x: (n % 3) as f32 * self.tile_size, y: self.tile_size * (n / 3) as f32, w: self.tile_size, h: self.tile_size} )))
                .collect::<Vec<Texture2D>>();
            self.textures = Some(sub_images);
        }
    }

    fn draw_numbered_tile(&self, tile: i32, x_pos: f32, y_pos: f32) {
        // Draws numbered tiles as text
        let size = 100;
        let txt = tile.to_string();
        let font_center = get_text_center(txt.as_str(), Option::None, size, 1.0, 0.0);
        draw_rectangle(x_pos, y_pos, self.tile_size, self.tile_size, GOLD);
        draw_text(txt.as_str(), x_pos + self.tile_size/2. - font_center.x, y_pos + self.tile_size/2. - font_center.y, size as f32, WHITE);
        draw_rectangle_lines(self.position.x + (tile % 3) as f32 * self.tile_size, self.position.y + (self.tile_size * (tile / 3) as f32), self.tile_size, self.tile_size, 1., BLACK);
    }

    pub fn draw(&self) {
        //draw_texture_ex(&self.texture, self.position.x, self.position.y, WHITE, DrawTextureParams {dest_size: Some(self.dimension), source: None, rotation: 0., flip_x: false, flip_y: false, pivot: None});
        //draw_rectangle_lines(49.5, 49.25, 500.5, 500.5, 5.0, BLACK);
        for tile in self.tiles {
            let x_pos = self.position.x + (tile % 3) as f32 * self.tile_size;
            let y_pos = self.position.y + (self.tile_size * (tile / 3) as f32);

            if self.draw_image_mode == true {
                if let Some(textures) = self.textures.as_ref() {
                    draw_texture(&textures[tile as usize], x_pos, y_pos, WHITE);		
                } else {
                    self.draw_numbered_tile(tile, x_pos, y_pos);
                }
            } else {
                    self.draw_numbered_tile(tile, x_pos, y_pos);
            }
        }
    }

    pub fn update(&mut self) {

        if is_key_pressed(KeyCode::Space) {
            self.draw_image_mode = !self.draw_image_mode;
        }
    }
}

