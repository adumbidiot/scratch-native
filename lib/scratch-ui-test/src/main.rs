extern crate scratch_ui;
use scratch_ui::{App, Sprite};

fn main() {
	let mut app = scratch_ui::PistonApp::new();
	app.init();
	
	//app.add_sprite(Sprite::new().path("assets/doge.jpg".to_string()));
	
	app.main_loop();
}
