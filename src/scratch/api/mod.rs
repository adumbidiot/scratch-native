pub mod targets;
pub mod types;

use self::types::*;
use curl::easy::{
    Easy2,
    Handler,
    WriteError,
};

pub struct Api {
    handle: Easy2<BufferBody>,
}

impl Api {
    pub fn new() -> Self {
        return Self {
            handle: Easy2::new(BufferBody::new()),
        };
    }

    pub fn get_url(&mut self, url: &str) -> ApiResult<Vec<u8>> {
        self.handle.url(&url)?;
        self.handle.perform()?;

        let handler = self.handle.get_mut();
        let body = handler.take_buffer();
        handler.reset();
        return Ok(body);
    }

    pub fn get_stats(&mut self, code: &str) -> ApiResult<InfoJson> {
        let url = format!("https://api.scratch.mit.edu/projects/{}/", code);
        let body = self.get_url(&url)?;
        let stats = serde_json::from_slice(&body).expect("Error parsing");
        return Ok(stats);
    }

    pub fn get_project_json(&mut self, code: &str) -> ApiResult<ProjectJson> {
        let url = format!(
            "https://cdn.projects.scratch.mit.edu/internalapi/project/{}/get",
            code
        );
        let body = self.get_url(&url)?;
        let project_json: ProjectJson = serde_json::from_slice(&body).expect("Error Parsing");
        return Ok(project_json);
    }

    pub fn get_project(&mut self, code: &str) -> ApiResult<Project> {
        let project_json = self.get_project_json(code)?;
        let mut project: Project = project_json.into();
        project.code = Some(code.to_string());

        let stats = self.get_stats(code)?;
        project.stats = Some(stats);

        return Ok(project);
    }

    pub fn get_asset(&mut self, name: &str) -> ApiResult<Vec<u8>> {
        let url = format!(
            "https://cdn.assets.scratch.mit.edu/internalapi/asset/{}/get",
            name
        );
        return self.get_url(&url);
    }
}

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    Fetch,
    Parse,
}

impl From<curl::Error> for ApiError {
    fn from(err: curl::Error) -> Self {
        ApiError::Fetch
    }
}

impl From<std::string::FromUtf8Error> for ApiError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        ApiError::Parse
    }
}

struct BufferBody {
    buffer: Vec<u8>,
}

impl BufferBody {
    pub fn new() -> Self {
        return Self { buffer: Vec::new() };
    }

    pub fn take_buffer(&mut self) -> Vec<u8> {
        let mut buffer = Vec::new();
        std::mem::swap(&mut self.buffer, &mut buffer);
        return buffer;
    }

    pub fn reset(&mut self) {
        self.buffer.truncate(0);
    }
}

impl Handler for BufferBody {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.buffer.extend_from_slice(data);
        return Ok(data.len());
    }
}
