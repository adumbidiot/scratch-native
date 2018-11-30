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
	pub x: i32,
	
	#[serde(rename = "scratchY")]
	pub y: i32,
	
	pub costumes: Vec<CostumeJson>
}