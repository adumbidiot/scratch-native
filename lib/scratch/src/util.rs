use serde::Serialize;
use std::{
    fs::File,
    path::PathBuf,
    process::Command,
};

pub struct FileCreater {
    pub path: PathBuf,
}

impl FileCreater {
    pub fn new(path: PathBuf) -> FileCreater {
        FileCreater { path }
    }

    pub fn exists(&mut self, path: &str) -> bool {
        self.path.push(path);
        let ret = self.path.exists();
        self.up();
        ret
    }

    pub fn is_dir(&self) -> bool {
        self.path.is_dir() //TODO: Return result to make everything one line??? idk
    }

    pub fn up(&mut self) -> &mut Self {
        self.path.pop();
        self
    }

    pub fn down(&mut self, path: &str) -> Result<&mut Self, ()> {
        self.path.push(path); //TODO: Resturn result, see if path exists or not
        if self.path.exists() {
            Ok(self)
        } else {
            self.up();
            Err(())
        }
    }

    pub fn mkdir(&mut self, path: &str) -> Result<&mut Self, std::io::Error> {
        //TODO: Consider custom error type
        self.path.push(path);
        std::fs::create_dir(&self.path).map(|_| self)
    }

    pub fn write_bytes(&mut self, name: &str, data: &[u8]) -> Result<&mut Self, std::io::Error> {
        self.path.push(name);
        std::fs::write(&self.path, data)?;
        self.path.pop();
        Ok(self)
    }

    pub fn write_json<T: Serialize>(
        &mut self,
        name: &str,
        data: T,
    ) -> Result<&mut Self, std::io::Error> {
        self.path.push(name);
        let mut file = File::create(&self.path)?;
        serde_json::to_writer_pretty(&mut file, &data).unwrap(); //TODO: Proper Error handling/return. Q: How does serialization fail anyways?
        self.path.pop();
        Ok(self)
    }
}

pub fn make_command() -> Command {
    let mut cmd = if cfg!(target_os = "windows") {
        Command::new("cmd")
    } else {
        Command::new("sh")
    };

    if cfg!(target_os = "windows") {
        cmd.arg("/C");
    } else {
        cmd.arg("-c");
    }
    cmd
}
