pub mod block;
pub mod json;

use serde::{
    Deserialize,
    Serialize,
};
use std::path::PathBuf;

pub use self::{
    super::*,
    block::*,
    json::*,
};

use self::super::{
    super::utils::DirCreater,
    targets::Target,
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Project {
    #[serde(default)]
    pub code: Option<String>,

    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub sound_list: Vec<SoundJson>,

    #[serde(default)]
    pub img_list: Vec<CostumeJson>,

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
        let mut project: Project =
            serde_json::from_slice(&main_buffer).expect("Could not parse Json");
        let project_json: ProjectJson =
            serde_json::from_slice(&data_buffer).expect("Could not parse Json");
        project.project_json = project_json;
        project.path = Some(path);
        return Ok(project);
    }

    pub fn get_name(&mut self) -> Option<&String> {
        return Some(
            self.name
                .get_or_insert("scratch_".to_string() + self.code.as_ref()?),
        );
    }

    pub fn get_save_path(&mut self, mut path: PathBuf) -> PathBuf {
        let name = self.get_name().expect("No Name");
        path.push(name);
        return path;
    }

    pub fn init(&mut self, api: &mut super::Api, mut path: PathBuf) -> ApiResult<()> {
        //Test to see if loc is valid?
        let mut builder = DirCreater::new(path.clone());
        builder
            .mkdir()
            .expect("Error making Dir")
            .down("assets")
            .mkdir()
            .expect("Error making assets dir")
            .down("images")
            .mkdir()
            .expect("Error making images dir")
            .up()
            .down("audio")
            .mkdir()
            .expect("Error making Audio Dir")
            .up()
            .up()
            .down("target")
            .mkdir()
            .expect("Error Making Target Dir")
            .up()
            .down("metadata")
            .mkdir()
            .expect("Error making metadata dir")
            .write_file(
                "project.json",
                &serde_json::to_vec_pretty(&self.project_json).expect("Error serializing"),
            )
            .expect("Error Saving metadata project.json")
            .up()
            .write_file(
                "project.json",
                &serde_json::to_vec_pretty(&self).expect("Error serializing"),
            )
            .expect("Error Saving project.json");

        builder.down("assets").down("audio");

        let mut asset_buf;
        for data in &self.sound_list {
            println!("Downloading: {}", &data.src);
            asset_buf = api.get_asset(&data.src).unwrap();
            builder
                .write_file(&data.src, &asset_buf)
                .expect("Error Saving file");
        }

        builder.up().down("images");

        for img in &self.img_list {
            println!("Downloading: {}", &img.src);
            asset_buf = api.get_asset(&img.src).unwrap();
            builder
                .write_file(&img.src, &asset_buf)
                .expect("Error Saving file");
        }

        builder.up().up();

        if let Some(stats) = self.stats.as_ref() {
            builder.down("metadata");
            builder
                .write_file(
                    "project_stats.json",
                    &serde_json::to_vec_pretty(&stats).expect("Error serializing"),
                )
                .expect("Could not save stats file");
        }

        return Ok(());
    }

    pub fn init_target<T: Target>(&mut self, target: &mut T) -> ApiResult<()> {
        let path_str = self.path.as_ref().expect("No Path");
        let mut path = PathBuf::from(path_str);
        path.push("target");
        path.push(target.get_name());
        target.init(self, &path).unwrap();
        return Ok(());
    }

    pub fn build_target<T: Target>(&mut self, target: &mut T) -> ApiResult<()> {
        let path_str = self.path.as_ref().expect("No Path");
        let mut path = PathBuf::from(path_str);
        path.push("target");
        path.push(target.get_name());
        target.build(self, &path).unwrap();
        return Ok(());
    }

    pub fn test_target<T: Target>(&mut self, target: &mut T) -> ApiResult<()> {
        let path_str = self.path.as_ref().expect("No Path");
        let mut path = PathBuf::from(path_str);
        path.push("target");
        path.push(target.get_name());
        target.test(self, &path).unwrap();
        return Ok(());
    }
}

impl From<ProjectJson> for Project {
    fn from(data: ProjectJson) -> Project {
        let mut p = Project::default();
        p.project_json = data;
        p.sound_list.extend(p.project_json.sounds.iter().cloned());

        let sprite_sounds: Vec<_> = p
            .project_json
            .children
            .iter()
            .flat_map(|child| {
                child
                    .sounds
                    .iter()
                    .cloned()
                    .filter(|el| p.sound_list.iter().find(|x| x.src == el.src).is_none())
            })
            .collect();
        p.sound_list.extend(sprite_sounds);

        p.img_list.extend(p.project_json.costumes.iter().cloned());
        let sprite_images: Vec<_> = p
            .project_json
            .children
            .iter()
            .flat_map(|child| {
                child
                    .costumes
                    .iter()
                    .cloned()
                    .filter(|el| p.img_list.iter().find(|x| x.src == el.src).is_none())
            })
            .collect();
        p.img_list.extend(sprite_images);

        return p;
    }
}
