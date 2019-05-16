use serde::{
    de::{
        SeqAccess,
        Visitor,
    },
    ser::SerializeSeq,
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ProjectInfoJson {
    comments_allowed: bool,
    //remix
    public: bool,
    //images
    description: String,
    id: u64,
    instructions: String,
    //history
    visibility: String,
    //author
    //stats
    image: String,
    is_published: bool,
    title: String,

    #[serde(flatten)]
    unknown: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectJson {
    //info
    pub children: Vec<SpriteJson>,
    pub costumes: Vec<CostumeJson>,
    #[serde(rename = "videoAlpha")]
    video_alpha: f32,
    #[serde(rename = "objName")]
    pub name: String,
    #[serde(rename = "currentCostumeIndex")]
    pub current_costume_index: u64,

    #[serde(flatten)]
    unknown: HashMap<String, serde_json::Value>,
}

impl ProjectJson {
    pub fn to_sprite_json(self) -> SpriteJson {
        SpriteJson {
            name: self.name,
            x: 0.0,
            y: 0.0,
            current_costume_index: self.current_costume_index,
            is_draggable: false,
            rotation_style: String::from("normal"),
            visible: true,
            scale: 1.0,
            direction: 90.0,
            costumes: self.costumes,
            index_in_library: 0,
            sounds: None,
            scripts: None,
            unknown: self.unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpriteJson {
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
    #[serde(rename = "indexInLibrary")]
    pub index_in_library: u32,

    pub sounds: Option<Vec<SoundJson>>,
    pub scripts: Option<Vec<ScriptJson>>,
    #[serde(flatten)]
    unknown: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CostumeJson {
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
    #[serde(flatten)]
    unknown: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoundJson {
    #[serde(rename = "soundName")]
    pub name: String,
    #[serde(rename = "md5")]
    pub src: String,
    #[serde(flatten)]
    unknown: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct ScriptJson {
    x: f32,
    y: f32,
    pub blocks: Vec<Block>,
}

impl<'de> Deserialize<'de> for ScriptJson {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(ScriptJsonVisitor)
    }
}

impl Serialize for ScriptJson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(3))?;
        seq.serialize_element(&self.x)?;
        seq.serialize_element(&self.y)?;
        seq.serialize_element(&self.blocks.iter().collect::<Vec<_>>())?;
        seq.end()
    }
}

struct ScriptJsonVisitor;

impl<'de> Visitor<'de> for ScriptJsonVisitor {
    type Value = ScriptJson;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Script Json")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<ScriptJson, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let x = seq
            .next_element()?
            .ok_or(serde::de::Error::invalid_length(0, &self))?;
        let y = seq
            .next_element()?
            .ok_or(serde::de::Error::invalid_length(1, &self))?;
        let blocks = seq
            .next_element()?
            .ok_or(serde::de::Error::invalid_length(2, &self))?;
        Ok(ScriptJson { x, y, blocks })
    }
}

#[derive(Debug, Clone)]
pub enum Block {
    WhenStart,
    PlaySoundAndWait(String),
    PlayNote(u32, f32),
    DoRepeat(u32, Vec<Block>),
    Unknown(Vec<serde_json::Value>),
}

impl<'de> Deserialize<'de> for Block {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(BlockVisitor)
    }
}

impl Serialize for Block {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Block::WhenStart => {
                let mut seq = serializer.serialize_seq(Some(1))?;
                seq.serialize_element("whenGreenFlag")?;
                seq.end()
            }
            Block::PlaySoundAndWait(sound) => {
                let mut seq = serializer.serialize_seq(Some(2))?;
                seq.serialize_element("doPlaySoundAndWait")?;
                seq.serialize_element(&sound)?;
                seq.end()
            }
            Block::PlayNote(note, duration) => {
                let mut seq = serializer.serialize_seq(Some(3))?;
                seq.serialize_element("noteOn:duration:elapsed:from:")?;
                seq.serialize_element(note)?;
                seq.serialize_element(duration)?;
                seq.end()
            }
            Block::DoRepeat(n, blocks) => {
                let mut seq = serializer.serialize_seq(Some(3))?;
                seq.serialize_element("doRepeat")?;
                seq.serialize_element(n)?;
                seq.serialize_element(blocks)?;
                seq.end()
            }
            Block::Unknown(arr) => {
                let mut seq = serializer.serialize_seq(Some(arr.len()))?;
                for e in arr {
                    seq.serialize_element(e)?;
                }
                seq.end()
            }
        }
    }
}

struct BlockVisitor;

impl<'de> Visitor<'de> for BlockVisitor {
    type Value = Block;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Block")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let name: String = seq
            .next_element()?
            .ok_or(serde::de::Error::invalid_length(0, &self))?;

        match name.as_str() {
            "whenGreenFlag" => Ok(Block::WhenStart),
            "doPlaySoundAndWait" => Ok(Block::PlaySoundAndWait(
                seq.next_element()?
                    .ok_or(serde::de::Error::invalid_length(1, &self))?,
            )),
            "noteOn:duration:elapsed:from:" => Ok(Block::PlayNote(
                seq.next_element()?
                    .ok_or(serde::de::Error::invalid_length(1, &self))?,
                seq.next_element()?
                    .ok_or(serde::de::Error::invalid_length(2, &self))?,
            )),
            "doRepeat" => Ok(Block::DoRepeat(
                seq.next_element()?
                    .ok_or(serde::de::Error::invalid_length(1, &self))?,
                seq.next_element()?
                    .ok_or(serde::de::Error::invalid_length(2, &self))?,
            )),
            _ => {
                dbg!(&name);
                let mut body = vec![serde_json::Value::String(name)];
                while let Some(data) = seq.next_element()? {
                    body.push(data);
                }

                Ok(Block::Unknown(body))
            }
        }
    }
}
