use crate::scratch::{
    PyGameIndexFile,
    ScratchError,
    ScratchResult,
};
pub use crate::scratch_crate::{
    client::Client,
    util::{
        make_command,
        FileCreater,
    },
    Project,
};
use scratch_crate::target::Target;
use std::process::Stdio;

fn test_import(data: &str) -> ScratchResult<bool> {
    Ok(make_command()
        .arg("python")
        .arg("-c")
        .arg(&format!("import {}", data))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .map_err(|_| ScratchError::Custom("Error Running Command".into()))?
        .status
        .success())
}

pub struct PyGameTarget {
    has_pygame: bool,
    has_pynanosvg: bool,
}

impl PyGameTarget {
    pub fn new() -> Self {
        PyGameTarget {
            has_pygame: false,
            has_pynanosvg: false,
        }
    }

    pub fn print_dep_stats(&self) {
        println!("Dependencies:");
        println!("------------------------");
        println!("PyGame [REQUIRED]: {}", self.has_pygame);
        println!("Pynanosvg: {}", self.has_pynanosvg);
        println!("\n")
    }
}

impl Target for PyGameTarget {
    fn name() -> &'static str {
        "python-pygame"
    }

    fn init(&mut self) -> ScratchResult<()> {
        self.has_pygame = test_import("pygame")?;
        let has_cython = test_import("cython")?;
        self.has_pynanosvg = test_import("svg")?;

        self.print_dep_stats();

        if !self.has_pygame {
            return Err(ScratchError::Custom("No Pygame Installed!".into()));
        }

        Ok(())
    }

    fn build(&mut self, project: &Project) -> ScratchResult<()> {
        let path = project.get_build_path(self);
        let mut file_creater = FileCreater::new(path);

        let mut index_file = PyGameIndexFile::new();
        index_file.add_sprite(
            &project
                .data
                .as_ref2()
                .expect("Scratch 2 Project")
                .clone()
                .to_sprite_json(),
        );
        for c in project
            .data
            .as_ref2()
            .expect("Scratch 2 Project")
            .children
            .iter()
        {
            index_file.add_sprite(c);
        }

        let _ = file_creater
            .write_bytes("index.py", &index_file.build().into_bytes())
            .map_err(|_| ScratchError::Custom("Error Writing index.py".into()))?
            .mkdir("assets")
            .is_ok();

        let mut client = Client::new();

        for c in project
            .data
            .as_ref2()
            .expect("Scratch 2 Project")
            .children
            .iter()
            .flat_map(|s| s.costumes.iter())
            .chain(
                project
                    .data
                    .as_ref2()
                    .expect("Scratch 2 Project")
                    .costumes
                    .iter(),
            )
        {
            if !file_creater.exists(&c.src) {
                println!("Downloading {}...", c.src);
                file_creater
                    .write_bytes(&c.src, &client.get_asset(&c.src)?)
                    .unwrap();
                if c.src.ends_with(".svg") {
                    dbg!("SVG");
                }
            }
        }

        for c in project
            .data
            .as_ref2()
            .expect("Scratch 2 Project")
            .children
            .iter()
            .flat_map(|s| s.sounds.iter())
            .flatten()
        {
            if !file_creater.exists(&c.src) {
                println!("Downloading {}...", c.src);
                file_creater
                    .write_bytes(&c.src, &client.get_asset(&c.src)?)
                    .unwrap();
            }
        }

        Ok(())
    }

    fn run(&mut self, project: &Project) -> ScratchResult<()> {
        let mut path = project.get_build_path(self);
        let data = make_command()
            .current_dir(&path)
            .arg("python")
            .arg("index.py")
            .stdout(Stdio::inherit())
            .output()
            .map_err(|_| ScratchError::Custom("Error Running Command".into()))?;
        dbg!(data);
        Ok(())
    }
}
