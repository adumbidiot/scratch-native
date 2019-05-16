use std::path::PathBuf;

pub struct DirCreater {
    path: PathBuf,
    logger: Box<DirCreaterLogger>,
}

impl DirCreater {
    pub fn new(path: PathBuf) -> DirCreater {
        return DirCreater {
            path,
            logger: Box::new(DefaultLogger::new()),
        };
    }

    pub fn with_logger<T: DirCreaterLogger + 'static>(&mut self, logger: T) -> &mut Self {
        self.logger = Box::new(logger);
        return self;
    }

    pub fn up(&mut self) -> &mut Self {
        self.path.pop();
        return self;
    }

    pub fn down(&mut self, path: &str) -> &mut Self {
        self.path.push(path);
        return self;
    }

    pub fn mkdir(&mut self) -> Result<&mut Self, std::io::Error> {
        self.logger.log_mkdir(&self.path);
        return std::fs::create_dir(&self.path).map(|_| self);
    }

    pub fn write_file(&mut self, name: &str, data: &[u8]) -> Result<&mut Self, std::io::Error> {
        self.path.push(name);
        self.logger.log_write_file(&self.path, name, data);
        let ret = std::fs::write(&self.path, data);
        self.path.pop();
        return ret.map(|_| self);
    }
}

pub trait DirCreaterLogger {
    fn new() -> Self
    where
        Self: Sized;
    fn log(&mut self);
    fn log_mkdir(&mut self, path: &PathBuf);
    fn log_write_file(&mut self, path: &PathBuf, name: &str, data: &[u8]);
}

struct DefaultLogger;

impl DirCreaterLogger for DefaultLogger {
    fn new() -> DefaultLogger {
        return DefaultLogger;
    }

    fn log(&mut self) {}

    fn log_mkdir(&mut self, path: &PathBuf) {
        println!("Making dir: {}", path.display());
    }

    fn log_write_file(&mut self, path: &PathBuf, _name: &str, _data: &[u8]) {
        println!("Creating: {}", path.display());
    }
}
