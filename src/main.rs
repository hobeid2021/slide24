use std::fs;
use std::convert::TryInto;
use macroquad::prelude::*;

#[derive(Debug)]
struct Puzzle {
    position: Vec2, // From top left corner
	dimension: Vec2,	
	tile_size: f32,
    tiles: [i32; 9],
	// TODO: Implement image textures as parts
	textures: Option<Vec<Texture2D>>,
}

impl Puzzle {
    pub fn new() -> Self {
        let tiles: [i32; 9] = (0..9).collect::<Vec<i32>>().try_into().unwrap();
		let dimension = Vec2::new(600.0, 600.0);
        Self {position: Vec2::new(50.,50.), dimension, tile_size: dimension.x / 3.0, tiles, textures: None}
    }

	pub async fn load_texture(&mut self, path: &str) {
        let puzzle_image = load_image("img/02.png").await.expect("Image not found"); 
		// Take slices of texture image
        let sub_images = self.tiles.into_iter().map(|n| Texture2D::from_image(&puzzle_image.sub_image(Rect {x: (n % 3) as f32 * self.tile_size, y: self.tile_size * (n / 3) as f32, w: self.tile_size, h: self.tile_size} )))
			.collect::<Vec<Texture2D>>();
		self.textures = Some(sub_images);
	}

	pub fn draw(&self) {
		//draw_texture_ex(&self.texture, self.position.x, self.position.y, WHITE, DrawTextureParams {dest_size: Some(self.dimension), source: None, rotation: 0., flip_x: false, flip_y: false, pivot: None});
        //draw_rectangle_lines(49.5, 49.25, 500.5, 500.5, 5.0, BLACK);
		for tile in self.tiles {
			let x_pos = self.position.x + (tile % 3) as f32 * self.tile_size;
			let y_pos = self.position.y + (self.tile_size * (tile / 3) as f32);
			let size = 100;
			match self.textures.as_ref() {
				Some(textures) => {
					draw_texture(&textures[tile as usize], x_pos, y_pos, WHITE);		
				},
				_ => {
					let txt = tile.to_string();
					let font_center = get_text_center(txt.as_str(), Option::None, size, 1.0, 0.0);
					draw_rectangle(x_pos, y_pos, self.tile_size, self.tile_size, GOLD);
					draw_text(txt.as_str(), x_pos + self.tile_size/2. - font_center.x, y_pos + self.tile_size/2. - font_center.y, size as f32, WHITE);
					draw_rectangle_lines(self.position.x + (tile % 3) as f32 * self.tile_size, self.position.y + (self.tile_size * (tile / 3) as f32), self.tile_size, self.tile_size, 1., BLACK);
				},
			}
		}
	}
}

#[macroquad::main("BasicShapes")]
async fn main() {
    request_new_screen_size(700., 700.);


    let mut puzzle = Puzzle::new();
	//puzzle.load_texture("").await;
    println!("{:?}", puzzle.tiles);
    loop {
        clear_background(GRAY);
        if is_key_down(KeyCode::Escape) {
            break;
        }        
		puzzle.draw();
        next_frame().await
    }
}
