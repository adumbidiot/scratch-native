use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectJson {
	#[serde(rename = "objName")]
	title: String,
	
	#[serde(rename = "tempoBPM")]
	tempo: u32,
	
	#[serde(rename = "videoAlpha")]
	alpha: f64,
	
	children: Vec<SpriteJson>,
	costumes: Vec<CostumeJson>,
}


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Project {
	tempo: u32,
	alpha: f64,
	pub children: Vec<Sprite>,
	pub costumes: Vec<Costume>,
	pub code: Option<String>,
}

impl Project {
	pub fn init(path: PathBuf){
		
	}
}

impl From<ProjectJson> for Project {
    fn from(data: ProjectJson) -> Project {
        let mut  p = Project{
			tempo: data.tempo,
			alpha: data.alpha,
			children: Vec::new(),
			costumes: Vec::new(),
			code: None
		};
		
		let children: Vec<Sprite> = data.children
			.into_iter()
			.map(|child|{
				return Sprite::from(child);
			})
			.collect();
			
		p.children = children;
		
		let costumes: Vec<Costume> = data.costumes
			.into_iter()
			.map(|child|{
				return Costume::from(child);
			})
			.collect();
			
		p.costumes = costumes;
		
		return p;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpriteJson{
	#[serde(rename = "objName")]
	name: String,
	
	#[serde(rename = "scratchX")]
	x: i32,
	
	#[serde(rename = "scratchY")]
	y: i32,
	
	costumes: Vec<CostumeJson>
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
			.map(|child|{
				return Costume::from(child);
			})
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CostumeJson{
	#[serde(rename = "costumeName")]
	name: String,
	
	#[serde(rename = "baseLayerMD5")]
	src: String,
	
	#[serde(rename = "bitmapResolution")]
	resolution: u8,
	
	#[serde(rename = "rotationCenterX")]
	center_x: i32,
	
	#[serde(rename = "rotationCenterY")]
	center_y: i32,
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