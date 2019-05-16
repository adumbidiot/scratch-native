use std::path::PathBuf;

use fs_extra::dir::{
    copy,
    CopyOptions,
};

use std::process::{
    Command,
    Stdio,
};

use self::super::{
    super::utils::DirCreater,
    types::{
        json::{
            ProjectJson,
            ScriptJson,
        },
        Block,
        Project,
    },
};

pub trait Target {
    fn get_name(&mut self) -> &str;
    fn init(&mut self, project: &mut Project, path: &PathBuf) -> TargetResult<()>;
    fn build(&mut self, project: &mut Project, path: &PathBuf) -> TargetResult<()>;
    fn test(&mut self, project: &mut Project, path: &PathBuf) -> TargetResult<()>;
}

#[derive(Debug)]
pub enum TargetError {
    FileAlreadyExists,
}

pub type TargetResult<T> = Result<T, TargetError>;

pub struct JsTarget {}

impl JsTarget {
    pub fn new() -> Self {
        JsTarget {}
    }

    pub fn write_file(path: &PathBuf, data: &[u8]) -> Result<(), std::io::Error> {
        println!("Creating {}", &path.display());
        std::fs::write(&path, data)
    }

    pub fn get_sprite_js(sprite: &super::types::json::SpriteJson) -> String {
        let costumes = sprite.costumes.iter().fold(String::new(), |code, costume| {
            code + &Self::get_costume_js(sprite, costume)
        });

        let mut scripts_str = String::new();
        if let Some(ref scripts) = sprite.scripts {
            for script in scripts.iter() {
                scripts_str += &Self::get_scratch_script_js(&script);
            }
        }

        return format!(
            r#"//Start {name}
let {name} = new Scratch.Sprite();
{name}.x = {x};
{name}.y = {y};
{costumes}
{name}.update = function(){{
	{scripts}
}};
//End {name}
"#,
            name = sprite.name,
            costumes = costumes,
            x = sprite.x,
            y = sprite.y,
            scripts = scripts_str
        );
    }

    pub fn get_costume_js(
        sprite: &super::types::json::SpriteJson,
        costume: &super::types::json::CostumeJson,
    ) -> String {
        return format!(
            r#"//Start {costume_name}
let {name}_{costume_name} = new Scratch.Costume();
{name}_{costume_name}.img.src = "assets/images/{src}";
{name}_{costume_name}.img.onload = function(){{
	{name}_{costume_name}.img.width = {name}_{costume_name}.img.width / {resolution};
	{name}_{costume_name}.img.height = {name}_{costume_name}.img.height / {resolution};
}};
{name}_{costume_name}.x = {x};
{name}_{costume_name}.y = {y};
{name}.costumes.push({name}_{costume_name});
//End {costume_name}
"#,
            name = sprite.name,
            costume_name = costume.name,
            src = costume.src,
            x = costume.center_x,
            y = costume.center_y,
            resolution = costume.resolution
        );
    }

    pub fn get_main_js(project: &ProjectJson) -> String {
        let mut sound_list: Vec<&_> = Vec::new();
        sound_list.extend(
            project.sounds.iter().chain(
                project
                    .children
                    .iter()
                    .flat_map(|child| child.sounds.iter()),
            ),
        );

        let mut audio_data = String::new();
        for audio in sound_list.iter() {
            audio_data += &format!(
                "game.audioAssets.set('{name}', new Audio('assets/audio/{md5}'));\n",
                name = audio.name,
                md5 = audio.src
            );
        }

        let mut body = project
            .children
            .iter()
            .map(|sprite| Self::get_sprite_js(sprite))
            .chain(
                project
                    .children
                    .iter()
                    .map(|sprite| format!("game.add({name});\n", name = sprite.name)),
            )
            .fold(String::new(), |body, data| body + &data);

        let main_js = format!(
            r#"//Auto-Generated by Scratch-Native
let game = new Scratch.Game("canvas");
{audio_data}
{body}
game.start();
"#,
            body = body,
            audio_data = audio_data
        );
        return main_js;
    }

    pub fn get_scratch_script_js(script: &ScriptJson) -> String {
        let mut total = String::new();
        let mut block_list = script.blocks.iter();
        if let Some(first) = block_list.next() {
            if let Block::WhenStart = first {
                for block in block_list {
                    total += &Self::get_scratch_block_js(&block);
                }
            }
        }
        return total;
    }

    pub fn get_scratch_block_js(block: &Block) -> String {
        let mut total = String::new();
        match block {
            Block::PlaySoundAndWait(ref data) => {
                total += &format!(
                    r#"if(game.data.get("GAME_FIRST_CYCLE")){{
		let audio = game.audioAssets.get('{name}');
		audio.play();
	}}"#,
                    name = data.song
                );
            }
            _ => {
                let ident = block.get_name();
                total += &format!(
                    r#"//{ident}
"#,
                    ident = ident
                );
            }
        }

        return total;
    }
}

impl Target for JsTarget {
    fn get_name(&mut self) -> &str {
        return "js";
    }

    fn init(&mut self, project: &mut Project, path: &PathBuf) -> TargetResult<()> {
        let name = project.get_name().unwrap().clone();
        println!("Setting up js target in {}", path.display());

        let main_js = Self::get_main_js(&project.project_json);

        let package_json = format!(
            r#"{{
  "name": "{}",
  "version": "0.0.1",
  "description": "Auto-Generated Scratch-Native Flash Game",
  "license": "MIT",
  "dependencies": {{
	
  }},
  "devDependencies" : {{
	"express": "~4.16.4"
  }},
  "scripts": {{
	"test": "node server.js"
  }}
}}"#,
            &name
        );

        let server_js = format!(
            r#"let express = require("express");
let path = require("path");
let app = express();
let PORT = 3001;
app.use(express.static("./"));
app.get("/js/scratch-js.js", function(req, res){{
	let runtime = path.join(__dirname, "..\\..\\..\\..\\lib\\scratch-js\\scratch-js.js")
	console.log("Sending runtime: " + runtime);
	res.sendFile(runtime);
}});

app.listen(PORT, function(){{
	console.log("Server running at " + PORT);
}});
"#
        );

        let index_html = format!(
            r#"<html>
	<head>
		<title>{title}</title>
		<script src="js/scratch-js.js"></script>
	</head>
	<body>
		<canvas id="canvas" width="480" height="360"></canvas>
		<script src="src/main.js"></script>
		<script>
			console.log("❗ Auto-Generated by scratch-native");
			console.log(game);
		</script>
	</body>
</html>
"#,
            title = &name
        );

        let mut dir_creater = DirCreater::new(path.clone());
        dir_creater
            .mkdir()
            .map_err(|_| TargetError::FileAlreadyExists)?
            .down("src")
            .mkdir()
            .expect("Could not make src dir")
            .write_file("main.js", &main_js.into_bytes())
            .expect("Could not save main.js")
            .up()
            .write_file("package.json", &package_json.into_bytes())
            .expect("Could not save package.json")
            .write_file("server.js", &server_js.into_bytes())
            .expect("Could not save server.js")
            .write_file("index.html", &index_html.into_bytes())
            .expect("Could not save index.html");

        let mut main_asset_dir = path.clone();
        main_asset_dir.pop();
        main_asset_dir.pop();
        main_asset_dir.push("assets");

        let mut options = CopyOptions::new();
        options.depth = 2;

        println!("Copying {} to {}", main_asset_dir.display(), path.display());
        copy(&main_asset_dir, &path, &options).unwrap();
        return Ok(());
    }

    fn build(&mut self, project: &mut Project, path: &PathBuf) -> TargetResult<()> {
        return Ok(());
    }

    fn test(&mut self, project: &mut Project, path: &PathBuf) -> TargetResult<()> {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .stdout(Stdio::inherit())
                .current_dir(&path)
                .arg("/C")
                .arg("npm test")
                .output()
                .expect("Error spawning command");
        } else {
            Command::new("sh")
                .stdout(Stdio::inherit())
                .current_dir(&path)
                .arg("-c")
                .arg("npm test")
                .output()
                .expect("Error spawning Command");
        }
        return Ok(());
    }
}
