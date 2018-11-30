pub mod json;

use std::path::PathBuf;

pub use self::json::*;
pub use self::super::*;
use self::super::targets::Target;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Project {
	#[serde(default)]
	pub code: Option<String>,
	
	#[serde(default)]
	pub name: Option<String>,
	
	#[serde(skip)]
	pub path: Option<PathBuf>,
	
	#[serde(skip)]
	pub project_json: ProjectJson,
}

impl Project {
	pub fn from_path(mut path: PathBuf) -> ApiResult<Self> {
		path.push("project.json");
		let main_buffer = std::fs::read(&path).expect("Could not read manifest");
		path.pop();
		path.push("metadata");
		path.push("project.json");
		let data_buffer = std::fs::read(&path).expect("Could not read data manifest");
		path.pop();
		path.pop();
		let mut project: Project = serde_json::from_slice(&main_buffer).expect("Could not parse Json");
		let project_json: ProjectJson = serde_json::from_slice(&data_buffer).expect("Could not parse Json");
		project.project_json = project_json;
		project.path = Some(path);
		return Ok(project);
	}
	
	pub fn get_name(&self) -> Option<&String>{
		return self.name.as_ref().or(self.code.as_ref());
	}
	
	fn mkdir(path: &PathBuf) -> Result<(), std::io::Error>{
		println!("Making dir: {}", path.display());
		return std::fs::create_dir(&path);
	}
	
	pub fn init(&self, api: &mut super::Api, mut path: PathBuf) -> ApiResult<()>{
		//Test to see if loc is valid
		let name = self.get_name().expect("No Name");
		path.push(name);
		Self::mkdir(&path).expect("Error making Directory");;
		
		
		let asset_dirs: [PathBuf; 2] = [
			"images".into(),
			"audio".into(),
		];
		
		let project_dirs: [PathBuf; 2] = [
			"target".into(),
			"metadata".into(),
		];
		
		{
			path.push("assets");
			Self::mkdir(&path).expect("Error making Directory");
			
			for rel_path in asset_dirs.iter(){
				path.push(rel_path);
				Self::mkdir(&path).expect("Error making Directory");;
				path.pop();
			}
			path.pop();
		}
		
		{
			for rel_path in &project_dirs {
				path.push(&rel_path);
				Self::mkdir(&path).unwrap();
				path.pop();
			}
		}
		
		path.push("assets/images");
		for child in self.project_json.children.iter() {
			for costume in child.costumes.iter() {
				let filename = &costume.src;
				println!("Downloading: {}", filename);
				let asset_buf = api.get_asset(filename).unwrap();
				path.push(filename);
				std::fs::write(&path, asset_buf).expect("Error writing asset file");
				path.pop();
			}
		}
		path.pop();
		path.pop();
		
		{
			path.push("assets/audio");
			path.pop();
			path.pop();
		}
		
		{
			path.push("project.json");
			let json = serde_json::to_vec_pretty(&self).expect("Error serializing");
			println!("Saving: {}", path.display());
			std::fs::write(&path, json).unwrap();
			path.pop();
		}
		
		{
			path.push("metadata");
			path.push("project.json");
			let json = serde_json::to_vec_pretty(&self.project_json).expect("Error serializing");
			println!("Saving: {}", path.display());
			std::fs::write(&path, json).unwrap();
			path.pop();
			path.pop();
		}

		return Ok(());
	}
	
	pub fn init_target<T: Target>(&self, target: &mut T) -> ApiResult<()>{
		let path_str = self.path.as_ref().expect("No Path");
		let mut path = PathBuf::from(path_str);
		path.push("target");
		path.push(target.get_name());
		target.init(&path).unwrap();
		return Ok(());
	}
	
	pub fn build_target<T: Target>(&self, target: &mut T) -> ApiResult<()>{
		
		return Ok(());
	}
}

impl From<ProjectJson> for Project {
    fn from(data: ProjectJson) -> Project {
        let mut p = Project::default();
		p.project_json = data;
		return p;
    }
}

impl From<SpriteJson> for Sprite {
    fn from(data: SpriteJson) -> Sprite {
        let mut s = Sprite {
			name: data.name,
			x: data.x,
			y: data.y,
			costumes: Vec::new(),
		};
		
		let costumes: Vec<Costume> = data.costumes
			.into_iter()
			.map(|child| Costume::from(child))
			.collect();
			
		s.costumes = costumes;
		
		return s;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sprite{
	pub name: String,
	pub x: i32,
	pub y: i32,
	pub costumes: Vec<Costume>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Costume{
	pub name: String,
	pub src: String,
	pub resolution: u8,
	pub center_x: i32,
	pub center_y: i32,
}

impl From<CostumeJson> for Costume {
    fn from(data: CostumeJson) -> Costume {
        Costume {
			name: data.name,
			src: data.src,
			resolution: data.resolution,
			center_x: data.center_x,
			center_y: data.center_y,
		}
    }
}