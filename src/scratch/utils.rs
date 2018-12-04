use std::path::PathBuf;

pub struct DirCreater{
	path: PathBuf
}

impl DirCreater{
	pub fn new(path: PathBuf) -> Self{
		return Self {
			path
		};
	}
	
	pub fn up(&mut self) -> &mut Self{
		self.path.pop();
		return self;
	}
	
	pub fn down(&mut self, path: &str) -> &mut Self{
		self.path.push(path);
		return self;
	}
	
	pub fn mkdir(&mut self) -> Result<&mut Self, std::io::Error>{
		println!("Making dir: {}", self.path.display());
		return std::fs::create_dir(&self.path).map(|_| self);
	}
	
	pub fn write_file(&mut self, name: &str, data: &[u8]) -> Result<&mut Self, std::io::Error>{
		self.path.push(name);
		println!("Creating: {}", self.path.display());
		let ret = std::fs::write(&self.path, data);
		self.path.pop();
		return ret.map(|_| self);
	}
}