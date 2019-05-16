pub mod client;
pub mod scratch3;
pub mod target;
pub mod types;
pub mod util;

use crate::{
    scratch3::ProjectJson as ProjectJson3,
    target::Target,
    types::ProjectJson as ProjectJson2,
    util::FileCreater,
};
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    borrow::Cow,
    path::PathBuf,
};

#[derive(Debug)]
pub enum ScratchError {
    Network,
    Json(serde_json::Error),
    Custom(Cow<'static, str>),
    InvalidSavePath,
}

pub type ScratchResult<T> = Result<T, ScratchError>;

pub struct SaveOptions {
    overwrite: bool,
}

impl SaveOptions {
    pub fn new() -> Self {
        SaveOptions { overwrite: false }
    }

    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }
}

#[derive(Serialize, Deserialize)]
pub enum ProjectData {
    Scratch2(ProjectJson2),
    Scratch3(ProjectJson3),
}

impl ProjectData {
    pub fn as_ref2(&self) -> Option<&ProjectJson2> {
        match self {
            ProjectData::Scratch2(data) => Some(data),
            ProjectData::Scratch3(_) => None,
        }
    }
}

pub struct Project {
    path: Option<PathBuf>,
    pub name: Option<String>,
    pub data: ProjectData,
}

impl Project {
    pub fn get_name(&self) -> &str {
        self.name.as_ref().map(|s| s.as_str()).unwrap_or("")
    }

    pub fn save(&mut self, path: PathBuf, options: SaveOptions) -> ScratchResult<()> {
        let mut file_creater = FileCreater::new(path);
        let name = self.get_name();

        if file_creater.is_dir() {
            if file_creater.down(name).is_ok() {
                if options.overwrite {
                    unimplemented!("Overwrite");
                } else {
                    return Err(ScratchError::InvalidSavePath);
                }
            } else {

            }
        } else {
            return Err(ScratchError::InvalidSavePath);
        }

        file_creater
            .mkdir(name)
            .map_err(|_| ScratchError::Custom("Error Creating Project Dir".into()))?
            .mkdir("data")
            .map_err(|_| ScratchError::Custom("Error Creating Data Dir!".into()))?
            .write_json("project.json", &self.data)
            .map_err(|_| ScratchError::Custom("Error Writing Project Json!".into()))?
            .up();

        self.path = Some(file_creater.path);

        Ok(())
    }

    pub fn load(mut path: PathBuf) -> ScratchResult<Self> {
        path.push("data");
        path.push("project.json");
        let file = std::fs::File::open(&path).unwrap();
        path.pop();
        path.pop();
        Ok(Project {
            path: Some(path),
            name: None, //TODO: METATDATA file
            data: serde_json::from_reader(&file).unwrap(),
        })
    }
    //TODO: Multiple build dirs? use type inference instead? keep for api similarity?
    pub fn get_build_path<T: Target>(&self, _target: &T) -> PathBuf {
        let mut path = self.path.as_ref().unwrap().clone();
        path.push("target");
        path.push(T::name());
        std::fs::create_dir_all(&path).unwrap();
        path
    }

    pub fn build<T: Target>(&self, target: &mut T) -> ScratchResult<()> {
        target.build(&self)
    }

    pub fn run<T: Target>(&self, target: &mut T) -> ScratchResult<()> {
        target.run(&self)
    }
}

impl From<ProjectJson2> for Project {
    fn from(data: ProjectJson2) -> Self {
        Project {
            data: ProjectData::Scratch2(data),
            name: None,
            path: None,
        }
    }
}

impl From<ProjectJson3> for Project {
    fn from(data: ProjectJson3) -> Self {
        Project {
            data: ProjectData::Scratch3(data),
            name: None,
            path: None,
        }
    }
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
        format!(
            "https://cdn.projects.scratch.mit.edu/internalapi/project/{}/get",
            self.id
        )
    }

    pub fn get_stats_url(&self) -> String {
        format!("https://api.scratch.mit.edu/projects/{}/", self.id)
    }
}
