use super::api::types::Sprite;

const PISTON_TEMPLATE_FRAG_1: &'static str = r#"
extern crate scratch_ui;
use scratch_ui::{App, Sprite};

fn main() {
	let mut app = scratch_ui::PistonApp::new();
	app.init();"#;
	
const PISTON_TEMPLATE_FRAG_2: &'static str = r#"	
	app.main_loop();
}
"#;

pub fn get_piston_app(input: &str) -> String{
	return format!("{}{}{}", PISTON_TEMPLATE_FRAG_1, input, PISTON_TEMPLATE_FRAG_2);
}

pub fn get_sprite(s: &Sprite) -> String{
	return format!(
	r#"
		app.add_sprite(Sprite::new()
			.path("assets//img//{}".to_string())
			.resolution({})
			.x({} as f64)
			.y({} as f64)
			.center({}, {})
		).unwrap();
	"#, s.costumes[0].src, s.costumes[0].resolution, s.x, s.y, s.costumes[0].center_x, s.costumes[0].center_y);
}