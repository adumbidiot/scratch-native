#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProjectJson {
	#[serde(rename = "objName")]
	pub title: String,
	
	#[serde(rename = "tempoBPM")]
	pub tempo: u32,
	
	#[serde(rename = "videoAlpha")]
	pub alpha: f64,
	
	pub children: Vec<SpriteJson>,
	pub costumes: Vec<CostumeJson>,
	
	#[serde(default)]
	pub sounds: Vec<SoundJson>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CostumeJson{
	#[serde(rename = "costumeName")]
	pub name: String,
	
	#[serde(rename = "baseLayerMD5")]
	pub src: String,
	
	#[serde(rename = "bitmapResolution")]
	pub resolution: u8,
	
	#[serde(rename = "rotationCenterX")]
	pub center_x: i32,
	
	#[serde(rename = "rotationCenterY")]
	pub center_y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpriteJson{
	#[serde(rename = "objName")]
	pub name: String,
	
	#[serde(rename = "scratchX")]
	pub x: f64,
	
	#[serde(rename = "scratchY")]
	pub y: f64,
	
	pub costumes: Vec<CostumeJson>,
	
	#[serde(default)]
	pub sounds: Vec<SoundJson>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoundJson{
	#[serde(rename = "soundName")]
	pub name: String,
	
	#[serde(rename = "md5")]
	pub src: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InfoJson{
	author: AuthorJson,
	comments_allowed: bool,
	description: String,
	history: HistoryJson,
	id: u64,
	image: String,
	instructions: String,
	is_published: bool,
	stats: StatsJson,
	title: String,
	visibility: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AuthorJson{
	id: u64,
	username: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HistoryJson{
	created: String,
	modified: String,
	shared: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct StatsJson{
	comments: u64,
	favorites: u64,
	loves: u64,
	remixes: u64,
	views: u64,
}