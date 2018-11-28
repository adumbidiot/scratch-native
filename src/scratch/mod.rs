mod error;
mod project;
mod templates;
pub mod api;

use hyper::rt::Future;
use hyper::rt::{Stream};
use std::path::PathBuf;

use self::error::Error;
use self::project::{Project, ProjectJson};
use tokio::fs::{create_dir_all};
use tokio::fs::file::{File};
use futures::future::{join_all};
use std::io::{Write, Read};
use std::process::Command;
use tokio_process::CommandExt;
use tokio::fs::OpenOptions;

use std::sync::RwLock;
use std::sync::Arc;


type Client = hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>;
const SCRATCH_ASSETS_CDN: &'static str = "https://cdn.assets.scratch.mit.edu";
/*
pub fn get_project_json(code: &str) -> Result<Project, Error>{
	let code_str = code.to_string();
	let api = api::Api::new();
	let mut project = api.get_project(code).unwrap();
	project.code = Some(code_str);
	return Ok(project);
*/

pub fn generate_project(client: &Client, project: &Project, project_path: &PathBuf) -> Result<impl Future<Item=(), Error=Error>, Error>{
	let mut project_path_mut = project_path.clone();
	let json = serde_json::to_vec_pretty(project)?;	
	match &project.code{
		Some(code) => {
			project_path_mut.push(code);
		},
		None => {
			project_path_mut.push("default");
		}
	}
	
	project_path_mut.push("assets/images");
		
	let mut image_futures = Vec::new();
	for child in project.children.iter(){
		for i in 0..child.costumes.len(){
			let filename = child.costumes[i].src.clone();
			let url = format!("{}/internalapi/asset/{}/get", SCRATCH_ASSETS_CDN, filename).parse()?;
			let get_img = client.get(url)
				.and_then(|req|{
					req.into_body().concat2()
				})
				.from_err::<Error>();
			
			project_path_mut.push(filename);
			let make_file = File::create(project_path_mut.clone())
				.from_err();
			project_path_mut.pop();
				
			let work = get_img
				.join(make_file)
				.and_then(|(body, mut file)|{
					file.write_all(&body)?;
					return Ok(());
				});
			
			image_futures.push(work);
		}
	}
	
	let make_images_dir = create_dir_all(project_path_mut.clone())
		.from_err::<Error>()
		.and_then(|_|{
			join_all(image_futures)
		})
		.from_err::<Error>();
	
	project_path_mut.pop();
	
	project_path_mut.push("sounds");
	let make_sounds_dir = create_dir_all(project_path_mut.clone())
		.from_err::<Error>();
	project_path_mut.pop();
	
	project_path_mut.pop();
	
	
	project_path_mut.push("project.json");
	let create_project_json = File::create(project_path_mut.clone())
		.and_then(move |mut file|{
			file.write_all(&json)?;
			return Ok(());
		})
		.from_err();
	project_path_mut.pop();
	
	let work = make_sounds_dir
		.join(make_images_dir)
		.and_then(|_|{
			return create_project_json;
		});
	return Ok(work);
}

pub fn build_project(project_path: &PathBuf) -> Result<impl Future<Item=(), Error=Error>, Error>{
	let mut project_path_mut = project_path.clone();
	//let project_name = project_path.file_stem().ok_or(())?.into_owned();
	
	project_path_mut.push("project.json");
		let get_project_json = File::open(project_path_mut.clone())
			.and_then(|file|{
				let project: Project = serde_json::from_reader(file)?;
				return Ok(project);
			});
	project_path_mut.pop();
	project_path_mut.push("assets");
		project_path_mut.push("images");
			let img_dir = project_path_mut.clone();
		project_path_mut.pop();
	project_path_mut.pop();
	project_path_mut.push("target");
		let make_target_dir = create_dir_all(project_path_mut.clone())
			.from_err();
		project_path_mut.push("rust");
			let init_path = project_path_mut.clone();
			let build_path = project_path_mut.clone();
			let make_target_subdir = create_dir_all(project_path_mut.clone())
				.from_err();
			project_path_mut.push("cargo.toml");
				let cargo_path = project_path_mut.clone();
				let cargo_path1 = project_path_mut.clone();
			project_path_mut.pop();
			project_path_mut.push("assets");
				project_path_mut.push("img");
					let new_img_dir = project_path_mut.clone();
				project_path_mut.pop();
			project_path_mut.pop();
			project_path_mut.push("src");
				project_path_mut.push("main.rs");
					let main_src = project_path_mut.clone();
				project_path_mut.pop();
			project_path_mut.pop();	
		project_path_mut.pop();
	project_path_mut.pop();
	
	let mut project_create_ref: Arc<RwLock<Project>> = Arc::new(RwLock::new(Default::default()));
	let mut project_build_ref = project_create_ref.clone();
	
	let work = make_target_dir
		.join(make_target_subdir)
		.and_then(|_|{
			return get_project_json;
		})
		.from_err()
		.and_then(move |json|{
			let mut p = project_create_ref.write().unwrap();
			let work = init_rust_project(init_path, &json);
			*p = json;
			return work;
		})
		.from_err::<Error>()
		.and_then(move |data|{
			println!("{:?}", data);
			
			let main = OpenOptions::new()
				.write(true)
				.truncate(true)
				.open(main_src);
			
			let cargo = OpenOptions::new()
				.read(true)
				.open(cargo_path);
			
			return Ok((main, cargo));
		})
		.from_err::<Error>()
		.and_then(|(src_fut, cargo_fut)|{
			let work = cargo_fut
				.from_err::<Error>()
				.and_then(|mut file|{
					let mut cargo_file = String::new();
					file.read_to_string(&mut cargo_file)?;
					
					let mut cargo_file: toml::Value = cargo_file.parse()?;
					cargo_file["dependencies"] = toml!{
						scratch-ui = {path = "../../../../lib/scratch-ui"}
					};
					
					let data = toml::to_vec(&cargo_file)?;
					
					let file = OpenOptions::new()
						.write(true)
						.truncate(true)
						.open(cargo_path1);
					return Ok((file, data));
				})
				.from_err::<Error>()
				.and_then(|(file_fut, toml_data)|{
					return file_fut
						.from_err::<Error>()
						.and_then(move |mut file|{
							file.write_all(&toml_data)?;
							return Ok(());
						});
				})
				.map(|_|{
						
				});
			return src_fut
				.from_err()
				.join(work)
				.from_err::<Error>();
		})
		.and_then(move |(mut src, _)|{
			let p = project_build_ref.read().unwrap();
			let mut sprite_code = String::new();
			for i in 0..(*p).children.len(){
				sprite_code.push_str(&templates::get_sprite(&(*p).children[i]));
			}
			
			println!("{}", &sprite_code);
			
			let src_str = templates::get_piston_app(&sprite_code);
			src.write_all(src_str.as_bytes())?;
			return Ok(());
		})
		.from_err::<Error>()
		.and_then(|_|{
			return copy_dir(img_dir, new_img_dir);
		})
		.and_then(|_|{
			return Ok(Command::new("cargo")
				.current_dir(build_path)
				.arg("build")
				.spawn_async()?);
		})
		.from_err::<Error>()
		.and_then(|fut|{
			return fut.from_err::<Error>();
		})
		.map(|data|{
			println!("{:?}", data);
		});
	
	return Ok(work);
}

pub fn init_rust_project(path: PathBuf, project: &Project) -> impl Future<Item=(), Error=Error>{
	let project_name = "scratch".to_string() + &project.code.clone().unwrap_or("default".to_string());
	return Command::new("cargo")
		.current_dir(path)
		.arg("init")
		.arg(".")
		.arg("--name")
		.arg(project_name)
		.output_async()
		.map(|data|{
			println!("{:#?}", data);
		})
		.from_err(); 
}

pub fn copy_dir(src: PathBuf, mut dest: PathBuf) -> impl Future<Item=(), Error=Error>{
	let make_dest_dir = create_dir_all(dest.clone());
	let copy_files = tokio::fs::read_dir(src)
		.flatten_stream()
		.for_each(move |entry|{
			if let Some(filename) = entry.path().file_name(){
				//println!("{:?}", entry.path());
				//println!("{:?}", filename);
				
				dest.push(filename);
				//println!("{:?}", &dest);
				std::fs::copy(entry.path(), &dest)?;
				dest.pop();
			}
			return Ok(());
		})
		.from_err();
		
	let work = make_dest_dir.and_then(|_|{
		return copy_files;
	});
	
	return work.from_err();
}