pub mod json;

use std::path::PathBuf;

pub use self::json::*;
pub use self::super::*;
use self::super::targets::Target;
use self::super::super::utils::DirCreater;

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
	
	#[serde(skip)]
	pub stats: Option<InfoJson>,
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
	
	pub fn get_name(&mut self) -> Option<&String>{
		return Some(self.name.get_or_insert("scratch_".to_string() + self.code.as_ref()?));
	}
	
	pub fn get_save_path(&mut self, mut path: PathBuf) -> PathBuf{
		let name = self.get_name().expect("No Name");
		path.push(name);
		return path;
	}
	
	pub fn init(&mut self, api: &mut super::Api, mut path: PathBuf) -> ApiResult<()>{
		let mut sound_list: Vec<&String> = Vec::new();
		sound_list.extend(self.project_json.sounds.iter().map(|el| &el.src));
		sound_list.extend(self.project_json.children.iter().flat_map(|child| child.sounds.iter().map(|el| &el.src)));
		
		let mut img_list: Vec<&String> = Vec::new();
		img_list.extend(self.project_json.costumes.iter().map(|el| &el.src));
		img_list.extend(self.project_json.children.iter().flat_map(|child| child.costumes.iter().map(|el| &el.src)));
		
		//Test to see if loc is valid?
		let mut builder = DirCreater::new(path.clone());
		builder
			.mkdir().expect("Error making Dir")
				.down("assets")
				.mkdir().expect("Error making assets dir")
					.down("images")
					.mkdir().expect("Error making images dir")
				.up()
					.down("audio")
					.mkdir().expect("Error making Audio Dir")
				.up()
			.up()
				.down("target")
				.mkdir().expect("Error Making Target Dir")
			.up()
				.down("metadata")
				.mkdir().expect("Error making metadata dir")
				.write_file("project.json", &serde_json::to_vec_pretty(&self.project_json).expect("Error serializing"))
				.expect("Error Saving metadata project.json")
			.up()
			.write_file("project.json", &serde_json::to_vec_pretty(&self).expect("Error serializing"))
			.expect("Error Saving project.json");
		
		builder
			.down("assets")
			.down("audio");
		let mut asset_buf = Vec::new();
		for filename in sound_list {
			println!("Downloading: {}", filename);
			asset_buf = api.get_asset(filename).unwrap();
			builder
				.write_file(filename, &asset_buf)
				.expect("Error Saving file");
		}
		
		builder
			.up()
			.down("images");
		
		for filename in img_list {
			println!("Downloading: {}", filename);
			asset_buf = api.get_asset(filename).unwrap();
			builder
				.write_file(filename, &asset_buf)
				.expect("Error Saving file");
		}
		
		builder
			.up()
			.up();
		
		if let Some(stats) = self.stats.as_ref(){
			builder.down("metadata");
			builder.write_file("project_stats.json", &serde_json::to_vec_pretty(&stats).expect("Error serializing"))
				.expect("Could not save stats file");
		}
		
		return Ok(());
	}
	
	pub fn init_target<T: Target>(&mut self, target: &mut T) -> ApiResult<()>{
		let path_str = self.path.as_ref().expect("No Path");
		let mut path = PathBuf::from(path_str);
		path.push("target");
		path.push(target.get_name());
		target.init(self, &path).unwrap();
		return Ok(());
	}
	
	pub fn build_target<T: Target>(&mut self, target: &mut T) -> ApiResult<()>{
		let path_str = self.path.as_ref().expect("No Path");
		let mut path = PathBuf::from(path_str);
		path.push("target");
		path.push(target.get_name());
		target.build(self, &path);
		return Ok(());
	}
	
	pub fn test_target<T: Target>(&mut self, target: &mut T) -> ApiResult<()>{
		let path_str = self.path.as_ref().expect("No Path");
		let mut path = PathBuf::from(path_str);
		path.push("target");
		path.push(target.get_name());
		target.test(self, &path);
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