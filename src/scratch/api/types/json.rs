use serde::{Deserializer, Deserialize, Serialize, Serializer};
use serde::de::{MapAccess, SeqAccess, Visitor};

use serde_json::Value;

use std::collections::HashMap;
use std::marker::PhantomData;

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
	
	#[serde(rename = "currentCostumeIndex")]
	pub current_costume_index: u64,
	
	#[serde(rename = "isDraggable")]
	is_draggable: bool,
	
	#[serde(rename = "rotationStyle")]
	rotation_style: String,
	
	pub visible: bool,
	pub scale: f64,
	pub direction: f64,
	
	pub costumes: Vec<CostumeJson>,
	
	#[serde(default)]
	pub sounds: Vec<SoundJson>,
	
	scripts: Option<Vec<ScriptJson>>,
	
	#[serde(flatten)]
    unknown: HashMap<String, Value>,
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

#[derive(Debug, Default, Serialize)]
pub struct ScriptJson{
	x: f64,
	y: f64,
	blocks: Vec<Vec<String>>
}

impl<'d> Deserialize<'d> for ScriptJson {
    fn deserialize<D>(deserializer: D) -> Result<ScriptJson, D::Error>
        where D: Deserializer<'d>
    {
		deserializer.deserialize_any(BlockListOrStruct)
    }
}

struct BlockListOrStruct;

impl<'de> Visitor<'de> for BlockListOrStruct {
        type Value = ScriptJson;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("Struct or Array")
        }
		
		fn visit_seq<A>(self, mut seq: A) -> Result<ScriptJson, A::Error>
		where
			A: SeqAccess<'de>,
        {
			let x: f64 = seq.next_element()?.ok_or(serde::de::Error::invalid_length(0, &self))?;
			let y: f64 = seq.next_element()?.ok_or(serde::de::Error::invalid_length(1, &self))?;
			let blocks: Vec<Vec<String>> = seq.next_element()?.ok_or(serde::de::Error::invalid_length(2, &self))?;
			return Ok(ScriptJson{
				x,
				y,
				blocks,
			});
        }
		
		fn visit_map<M>(self, mut map: M) -> Result<ScriptJson, M::Error>
            where M: serde::de::MapAccess<'de>
        {
			let mut x = None;
			let mut y = None;
			let mut blocks = None;
			while let Some(key) = map.next_key()? {
				match key {
					"x" => {
						x = Some(map.next_value()?);
					},
					"y" => {
						y = Some(map.next_value()?);
					},
					"blocks" => {
						blocks = Some(map.next_value()?);
					},
					_=>{
					
					}
                }
            }
			
			let x = x.unwrap();
			let y = y.unwrap();
			let blocks = blocks.unwrap();
            return Ok(ScriptJson{
				x,
				y,
				blocks,
			});
        }
    }