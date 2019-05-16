use serde::{
    Deserialize,
    Serialize,
};
use std::collections::HashMap;

fn default_size() -> f64 {
    0.0
}

fn default_direction() -> u32 {
    0
}

#[derive(Debug)]
pub struct NetworkProject {
    id: String,
}

impl NetworkProject {
    pub fn new(id: String) -> Self {
        NetworkProject { id }
    }

    pub fn get_data_url(&self) -> String {
        format!("https://projects.scratch.mit.edu/309320008/{}/", self.id)
    }

    pub fn get_stats_url(&self) -> String {
        format!("https://api.scratch.mit.edu/projects/{}/", self.id)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectJson {
    extensions: Vec<serde_json::Value>,
    meta: serde_json::Value,
    monitors: Vec<serde_json::Value>,
    pub targets: Vec<TargetJson>,

    #[serde(flatten)]
    unknown: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TargetJson {
    pub blocks: HashMap<String, BlockJson>,
    broadcasts: serde_json::Value,
    comments: serde_json::Value,
    pub costumes: Vec<CostumeJson>,
    #[serde(rename = "currentCostume")]
    pub current_costume: u32,
    #[serde(default = "default_direction")]
    pub direction: u32,
    draggable: Option<bool>,
    #[serde(rename = "isStage")]
    is_stage: bool,
    #[serde(rename = "layerOrder")]
    layer_order: u32,
    lists: serde_json::Value,
    pub name: String,
    #[serde(rename = "rotationStyle")]
    rotation_style: Option<String>,
    #[serde(default = "default_size")]
    pub size: f64,
    pub sounds: Vec<SoundJson>,
    variables: serde_json::Value,
    visible: Option<bool>,
    volume: u32,

    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,

    #[serde(flatten)]
    unknown: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostumeJson {
    #[serde(rename = "assetId")]
    asset_id: String,
    #[serde(rename = "rotationCenterX")]
    pub rotation_center_x: f64,
    #[serde(rename = "rotationCenterY")]
    pub rotation_center_y: f64,
    #[serde(rename = "dataFormat")]
    data_format: String,
    #[serde(rename = "bitmapResolution")]
    pub bitmap_resolution: Option<f64>,
    name: String,
    pub md5ext: String,

    #[serde(flatten)]
    unknown: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoundJson {
    #[serde(rename = "assetId")]
    asset_id: String,
    #[serde(rename = "dataFormat")]
    data_format: String,
    format: serde_json::Value,
    pub md5ext: String,
    pub name: String,
    rate: u32,
    #[serde(rename = "sampleCount")]
    sample_count: u32,

    #[serde(flatten)]
    unknown: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockJson {
    pub fields: HashMap<String, Vec<serde_json::Value>>,
    parent: Option<String>,
    pub next: Option<String>,
    #[serde(rename = "topLevel")]
    pub top_level: bool,
    shadow: bool,
    pub opcode: String,
    pub inputs: HashMap<String, Vec<serde_json::Value>>,
    x: Option<u32>,
    y: Option<u32>,

    #[serde(flatten)]
    unknown: HashMap<String, serde_json::Value>,
}
