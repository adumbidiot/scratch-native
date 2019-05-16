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
use curl::easy::{
    Easy2,
    Handler,
    WriteError,
};

pub struct Client {
    handle: Easy2<BufferBody>,
}

impl Client {
    pub fn new() -> Self {
        Client {
            handle: Easy2::new(BufferBody::new()),
        }
    }

    pub fn get_url(&mut self, url: &str) -> ScratchResult<Vec<u8>> {
        self.handle.url(&url).unwrap();
        self.handle.perform().map_err(|_| {
            self.handle.get_mut().reset();
            ScratchError::Network
        })?;

        let handler = self.handle.get_mut();
        let body = handler.take_buffer();
        handler.reset();
        Ok(body)
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

struct BufferBody {
    buffer: Vec<u8>,
}

impl BufferBody {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn take_buffer(&mut self) -> Vec<u8> {
        let mut buffer = Vec::new();
        std::mem::swap(&mut self.buffer, &mut buffer);
        buffer
    }

    pub fn reset(&mut self) {
        self.buffer.truncate(0);
    }
}

impl Handler for BufferBody {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.buffer.extend_from_slice(data);
        Ok(data.len())
    }
}
