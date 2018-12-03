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
use std::io::Write;

use clap::{App, Arg, SubCommand};

mod scratch;

fn main() {
	let matches = App::new("scratch-native")
        .subcommand(SubCommand::with_name("new")
            .arg(Arg::with_name("path")
                .required(true)) //Remove for current dir?
			.arg(Arg::with_name("code")
				.takes_value(true)
				.short("c")
				.long("code")
				.required(true))) //temp
		.subcommand(SubCommand::with_name("info")
			.arg(Arg::with_name("code")
                .required(true)) //Allow paths? Optional args?
			.arg(Arg::with_name("type")
                .takes_value(true)
				.short("t")
				.long("type")))
		.get_matches();
	
	let mut api = scratch::api::Api::new();
	
	match matches.subcommand() {
		("new", Some(matches)) => {
            let path = PathBuf::from(matches.value_of("path").expect("No path specified"));
			let code = matches.value_of("code").expect("No Project Code Specified");
			
			println!("Downloading Project... ");
			let mut project = api.get_project(code).expect("Error getting project json");
			let save_path = project.get_save_path(path);
			
			if save_path.exists(){
				let mut output = String::new();
				println!("The directory '{}' already exists.", save_path.display());
				print!("Overwrite? (Y/N): ");
				std::io::stdout().flush().unwrap();
				std::io::stdin().read_line(&mut output).unwrap();
				
				let output_char = output.to_uppercase().chars().next().unwrap();
				if output_char == 'N' {
					println!("Aborting...");
					return;
				}else if output_char == 'Y' {
					println!("Deleting Directory...");
					std::fs::remove_dir_all(&save_path).unwrap();
				}else{
					println!("Unknown Input. Aborting...");
					return;
				}
			}
			println!("Generating Project... ");
			project.init(&mut api, save_path).expect("Error saving project");
		},
		("info", Some(matches)) => {
			let code = matches.value_of("code").expect("Code not specified");
			if let Some(data_type) = matches.value_of("type"){
				match data_type {
					"all" => {
						let mut project = api.get_project(code).expect("Error getting project json");
						println!("{:#?}", project);
					},
					"json" => {
						let mut project = api.get_project_json(code).expect("Error getting project json");
						println!("{:#?}", project);
					},
					"stats" => {
						let stats = api.get_stats(code).expect("Error getting project json");
						println!("{:#?}", stats);
					},
					_=> {
						println!("type {} is not valid", &data_type);
					}	
				}
			}else{
				let mut project = api.get_stats(code).expect("Error getting project json");
				println!("{:#?}", project);
			}
		},
		("build", Some(matches)) => {
			
		},
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(), 
    }
}

//Hack to keep old code
fn old_main(){
	std::fs::remove_dir_all("projects/239742347/target/js");
	println!("Loading Project... ");
	let mut path = PathBuf::from("projects");
	path.push("239742347");
	let mut project = scratch::api::types::Project::from_path(path).expect("Error Loading Project");
	let mut target = scratch::api::targets::JsTarget::new();
	println!("Setting up target... ");
	project.init_target(&mut target).unwrap();
	println!("Building target... ");
	project.build_target(&mut target).unwrap();
	println!("Testing target... ");
	project.test_target(&mut target).unwrap();
	
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
