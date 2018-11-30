use std::path::PathBuf;

use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;
use fs_extra::dir::create;

pub trait Target {
	fn get_name(&mut self) -> &str;
	fn init(&mut self, path: &PathBuf) -> TargetResult<()>;
}

#[derive(Debug)]
pub enum TargetError {
	FileAlreadyExists
}

pub type TargetResult<T> = Result<T, TargetError>;

pub struct JsTarget {
	
}

impl JsTarget {
	pub fn new() -> Self {
		JsTarget {
			
		}
	}	
}

impl Target for JsTarget {
	fn get_name(&mut self) -> &str{
		return "js";
	}
	
	fn init(&mut self, path: &PathBuf) -> TargetResult<()>{
		println!("Setting up js target in {}", path.display());
		std::fs::create_dir(path).map_err(|_| TargetError::FileAlreadyExists)?;
		
		let mut target_dir = path.clone();
		//create(&target_asset_dir, false).expect("Could not make target asset dir");
		
		let mut main_asset_dir = path.clone();
		main_asset_dir.pop();
		main_asset_dir.pop();
		main_asset_dir.push("assets");
		
		let mut options = CopyOptions::new();
		options.depth = 2;
		
		println!("Copying {} to {}", main_asset_dir.display(), target_dir.display());
		copy(&main_asset_dir, &target_dir, &options).unwrap();
		
		target_dir.push("src");
		create(&target_dir, false).expect("Could not make src dir");
		
		target_dir.pop();
		
		return Ok(());
	}
}