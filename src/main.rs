extern crate hyper;
extern crate tokio;
extern crate tokio_process;
#[macro_use]
extern crate toml;

extern crate curl;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate fs_extra;

use hyper::rt::{self, Future};

use std::path::PathBuf;

mod scratch;

fn main() {	
	std::fs::remove_dir_all("projects/239742347/target/js").unwrap();
	println!("Loading Project... ");
	let mut path = PathBuf::from("projects");
	path.push("239742347");
	let project = scratch::api::types::Project::from_path(path).expect("Error Loading Project");
	let mut target = scratch::api::targets::JsTarget::new();
	println!("Setting up target... ");
	project.init_target(&mut target).unwrap();
	println!("Building target... ");
	project.build_target(&mut target).unwrap();
	
	return;
	std::fs::remove_dir_all("projects/239742347").unwrap();
	
	let mut api = scratch::api::Api::new();
	
	println!("Downloading Project... ");
	let mut project = api.get_project("239742347").unwrap();
	
	println!("Generating Project... ");
	project.init(&mut api, "projects".into()).unwrap();
	
	
	return;
	
	print!("Building Project... ");
	/*
	let work = scratch::build_project(&std::path::PathBuf::from("projects\\239742347"))
		.unwrap()
		.map(|_|{
			println!("Done");
		})
		.map_err(|err|{
			println!("Error: {:#?}", err);
		});
	
	rt::run(work);
	*/
}
