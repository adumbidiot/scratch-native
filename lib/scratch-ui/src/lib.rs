extern crate piston_window;
extern crate image;

use piston_window::{PistonWindow, WindowSettings, clear, rectangle, G2dTexture, Texture, Flip, TextureSettings, image as draw_image};

use piston_window::Transformed;

use image::GenericImageView;

pub trait App{
	fn new() -> Self;
	fn init(&mut self);
	fn add_sprite(&mut self, s: Sprite) -> Result<usize, Error>;
	fn main_loop(&mut self);
}

pub struct PistonApp{
	window: Option<PistonWindow>,
	sprites: Vec<PistonSprite>,
}

impl App for PistonApp{
	fn new() -> PistonApp{
		return PistonApp {
			sprites: Vec::new(),
			window: None
		};
	}
	
	fn init(&mut self){
		let window = WindowSettings::new("Default", [480, 360])
			.exit_on_esc(true)
			.build()
			.unwrap();
		self.window = Some(window);
	}
	
	fn add_sprite(&mut self, s: Sprite) -> Result<usize, Error>{
		let window = self.window.as_mut().ok_or(Error::WindowNotFound)?;
		
		let img = image::open(&s.path).unwrap();
		
		let width = img.width() / s.resolution;
		let height = img.height() / s.resolution;
		
		let scaled = img.resize(width , height, image::FilterType::Nearest);
		
		let sprite_texture: G2dTexture = Texture::from_image(
            &mut window.factory,
            &scaled.to_rgba(),
            &TextureSettings::new()
        ).unwrap();
		
		let stage_center_x = 480.0 / 2.0;
		let stage_center_y = 360.0 / 2.0;
		
		
		
		let center_x =  (480.0 - s.center_x as f64) / 2.0;
		let center_y =  (360.0 - s.center_y as f64) / 2.0;
		
		println!("({},{})", s.x, s.y);
		println!("({},{})", center_x, center_y);
		
		let sprite = PistonSprite{
			x: center_x + s.x,
			y: center_y - s.y,
			sprite: sprite_texture, 
		};
		
		self.sprites.push(sprite);
		return Ok(self.sprites.len() - 1);
	}
	
	fn main_loop(&mut self){
		let window = self.window.as_mut().unwrap();
		let sprites = &self.sprites;
		
		while let Some(event) = window.next(){
			window.draw_2d(&event, |context, graphics| {
				clear([1.0; 4], graphics);
				rectangle([1.0, 0.0, 0.0, 0.01], // Bug on clearing with no bjects renderd
                     [0.0, 1.0, 1.0, 1.0],
                     context.transform,
                      graphics);
				
				for i in 0..sprites.len(){
					let transform = context.transform.trans(sprites[i].x, sprites[i].y);
					draw_image(&sprites[i].sprite, transform, graphics);
				}
			});
		}
	}
}

pub struct Sprite{
	x: f64,
	y: f64,
	resolution: u32,
	path: String,
	center_x: i32,
	center_y: i32,
}

impl Sprite{
	pub fn new() -> Self{
		return Sprite{
			x: 0.0, 
			y: 0.0,
			resolution: 1,
			path: String::new(),
			center_x: 0,
			center_y: 0,
		}
	}
	
	pub fn x(mut self, x: f64) -> Self{
		self.x = x;
		return self;
	}
	
	pub fn y(mut self, y: f64) -> Self{
		self.y = y;
		return self;
	}
	
	pub fn path(mut self, path: String) -> Self {
		self.path = path;
		return self;
	}
	
	pub fn resolution(mut self, resolution: u32) -> Self {
		self.resolution = resolution;
		return self;
	}
	
	pub fn center(mut self, x: i32, y: i32) -> Self {
		self.center_x = x;
		self.center_y = y;
		return self;
	}
}

pub struct PistonSprite{
	sprite: G2dTexture,
	x: f64,
	y: f64,
}

#[derive(Debug)]
pub enum Error{
	FileNotFound(String),
	WindowNotFound,
}
