pub mod types;

use curl::easy::{Easy2, Handler, WriteError};
use self::types::*;

pub struct Api {
	handle: Easy2<BufferBody>,
}

impl Api {
	pub fn new() -> Self {
		return Self {
			handle: Easy2::new(BufferBody::new())
		};
	}
	
	pub fn get_project(&mut self, code: &str) -> ApiResult<Project>{
		let url = format!("https://cdn.projects.scratch.mit.edu/internalapi/project/{}/get", code);
		
		self.handle.url(&url)?;
		self.handle.perform()?;
		
		let handler = self.handle.get_mut();
		let body = handler.take_buffer();
		handler.reset();
		
		let project: ProjectJson = serde_json::from_slice(&body).expect("Error Parsing");
		
		return Ok(project.into());
	}
}

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
	Fetch,
	Parse
}

impl From<curl::Error> for ApiError{
    fn from(err: curl::Error) -> Self {
        ApiError::Fetch
    }
}

impl From<std::string::FromUtf8Error> for ApiError{
    fn from(err: std::string::FromUtf8Error) -> Self {
        ApiError::Parse
    }
}

struct BufferBody {
	buffer: Vec<u8>,
}

impl BufferBody {
	pub fn new() -> Self{
		return Self {
			buffer: Vec::new(),
		}
	}
	
	pub fn take_buffer(&mut self) -> Vec<u8>{
		let mut buffer = Vec::new();
		std::mem::swap(&mut self.buffer, &mut buffer);
		return buffer;
	}
	
	pub fn reset(&mut self){
		self.buffer.truncate(0);
	}
}

impl Handler for BufferBody {
	fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.buffer.extend_from_slice(data);
        return Ok(data.len());
    }
}