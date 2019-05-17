use crate::{
    scratch3::{
        NetworkProject as NetworkProject3,
        ProjectJson as ProjectJson3,
    },
    types::{
        ProjectInfoJson,
        ProjectJson,
    },
    NetworkProject,
    ScratchError,
    ScratchResult,
};

pub struct Client {
    handle: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Client {
            handle: reqwest::Client::new(),
        }
    }

    pub fn get_url(&mut self, url: &str) -> ScratchResult<Vec<u8>> {
        let mut buf = Vec::new();
        self.handle
            .get(url)
            .send()
            .map_err(|_e| ScratchError::Network)?
            .copy_to(&mut buf)
            .map_err(|_e| ScratchError::Network)?;
        Ok(buf)
    }

    pub fn get_stats(&mut self, project: &NetworkProject) -> ScratchResult<ProjectInfoJson> {
        serde_json::from_slice(&self.get_url(&project.get_stats_url())?)
            .map_err(|e| ScratchError::Json(e))
    }

    pub fn get_data(&mut self, project: &NetworkProject) -> ScratchResult<ProjectJson> {
        serde_json::from_slice(&self.get_url(&project.get_data_url())?)
            .map_err(|e| ScratchError::Json(e))
    }

    pub fn get_data_3(&mut self, project: &NetworkProject3) -> ScratchResult<ProjectJson3> {
        serde_json::from_slice(&self.get_url(&project.get_data_url())?)
            .map_err(|e| ScratchError::Json(e))
    }

    pub fn get_asset(&mut self, file: &str) -> ScratchResult<Vec<u8>> {
        let url = format!(
            "https://cdn.assets.scratch.mit.edu/internalapi/asset/{}/get",
            file
        );
        self.get_url(&url)
    }
}
