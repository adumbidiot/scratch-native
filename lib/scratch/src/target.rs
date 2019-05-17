use crate::{
    client::Client,
    util::{
        make_command,
        FileCreater,
    },
    Project,
    ProjectData,
    ScratchError,
    ScratchResult,
};
use std::process::Stdio;

mod python;

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

pub trait Target {
    fn name() -> &'static str;
    fn init(&mut self) -> ScratchResult<()>;
    fn build(&mut self, project: &Project) -> ScratchResult<()>;
    fn run(&mut self, project: &Project) -> ScratchResult<()>;
}

pub struct PyGameTarget {
    has_pygame: bool,
    has_pynanosvg: bool,
}

struct CodeGen {
    data: String,
    tab_index: usize,
}

impl CodeGen {
    pub fn new() -> Self {
        CodeGen {
            data: String::new(),
            tab_index: 0,
        }
    }

    pub fn writeln(&mut self, data: &str) {
        self.data += &"\t".repeat(self.tab_index);
        self.data += data;
        self.data.push('\n');
    }
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
        println!("PyGame: {}", self.has_pygame);
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
        let _has_cython = test_import("cython")?;
        self.has_pynanosvg = test_import("svg")?;

        self.print_dep_stats();

        if !self.has_pygame {
            return Err(ScratchError::Custom("No Pygame Installed!".into()));
        }

        Ok(())
    }

    fn build(&mut self, project: &Project) -> ScratchResult<()> {
        let mut client = Client::new();
        let path = project.get_build_path(self);
        let mut file_creater = FileCreater::new(path);

        match &project.data {
            ProjectData::Scratch3(data) => {
                let mut index = String::new();
                index += "import pygame\n";
                index += "import pygame.midi\n";
                index += "import time\n";
                index += "import types\n";
                index += "from svg import Parser, Rasterizer\n";
                index += include_str!("./target/event.py");
                index += include_str!("./target/event_dispatcher.py");
                index += include_str!("./target/costume.py");
                index += include_str!("./target/sprite.py");
                index += "pygame.init()\n";
                index += "pygame.midi.init()\n";
                index += "midi_player = pygame.midi.Output(0)\n";
                index += "midi_player.set_instrument(0)\n";
                index += "sprite_list = []\n";
                index += "event_system = EventDispatcher()\n";
                index += "sound_list = {}\n";
                index += "block_list = {}\n";

                for s in data.targets.iter().flat_map(|t| t.sounds.iter()) {
                    index += &format!(
                        "sound_list['{}'] = pygame.mixer.Sound('assets/{}')\n",
                        s.name, s.md5ext
                    );
                }

                for (i, target) in data.targets.iter().enumerate() {
                    let name = format!("{}_{}", target.name, i);
                    index += &format!(
                        "{name} = Sprite({x}, {y}, {costume}, {direction}, {})\n",
                        target.size,
                        x = 0,
                        y = 0,
                        name = name,
                        costume = target.current_costume,
                        direction = target.direction
                    );
                    for costume in target.costumes.iter() {
                        index += &format!(
                            "{name}.costumes.append(Costume({x}, {y}, {}, 'assets/{src}'))\n",
                            costume.bitmap_resolution.unwrap_or(1.0),
                            x = costume.rotation_center_x,
                            y = costume.rotation_center_y,
                            src = costume.md5ext,
                            name = name
                        );
                    }

                    for (i, (id, block)) in target.blocks.iter().enumerate() {
                        let mut codegen = CodeGen::new();
                        match block.opcode.as_str() {
                            "event_whenflagclicked" => {
                                codegen.writeln(&format!("def block_{}(e):", i));
                                codegen.tab_index += 1;
                            }
                            "control_forever" => {
                                codegen.writeln(&format!("def block_{}(e):", i));
                                codegen.tab_index += 1;
                                codegen.writeln("while True:");
                                codegen.tab_index += 1;
                                codegen.writeln(&format!(
                                    "block = block_list[{}](e)",
                                    block.inputs["SUBSTACK"][1]
                                ));
                                codegen.writeln("if isinstance(block, types.GeneratorType):");
                                codegen.tab_index += 1;
                                codegen.writeln(&format!("yield from block"));
                                codegen.tab_index -= 1;
                            }
                            "sound_playuntildone" => {
                                codegen.writeln(&format!("def block_{}(e):", i));
                                codegen.tab_index += 1;
                                let sound_name = target.blocks
                                    [block.inputs["SOUND_MENU"][1].as_str().unwrap()]
                                .fields["SOUND_MENU"][0]
                                    .as_str()
                                    .unwrap();
                                codegen.writeln(&format!("sound_list['{}'].play()", sound_name));
                                codegen.writeln("t0 = time.time()");
                                codegen.writeln(&format!(
                                    "while time.time() - t0 < sound_list['{}'].get_length():",
                                    sound_name
                                ));
                                codegen.writeln("\tyield");
                            }
                            "sound_changevolumeby" => {
                                codegen.writeln(&format!("def block_{}(e):", i));
                                codegen.tab_index += 1;
                                codegen.writeln("for sound in sound_list:");
                                codegen.tab_index += 1;
                                codegen.writeln("current_volume = sound_list[sound].get_volume()");
                                let change = block.inputs["VOLUME"][1][1].as_str().unwrap();
                                codegen.writeln(&format!(
                                    "sound_list[sound].set_volume(current_volume + {})",
                                    change
                                ));
                                codegen.tab_index -= 1;
                            }
                            "motion_goto" => {
                                codegen.writeln(&format!("def block_{}(e):\n", i));
                                codegen.tab_index += 1;
                                let loc = target.blocks[block.inputs["TO"][1].as_str().unwrap()]
                                    .fields["TO"][0]
                                    .as_str()
                                    .unwrap();
                                match loc {
                                    "_mouse_" => {
                                        codegen.writeln("pos = pygame.mouse.get_pos()");
                                        codegen
                                            .writeln(&format!("{}.x = -(480 / 2) + pos[0]", name));
                                        codegen
                                            .writeln(&format!("{}.y = (360 / 2) - pos[1]", name));
                                    }
                                    _ => unimplemented!(),
                                }
                            }
                            "motion_turnright" => {
                                codegen.writeln(&format!("def block_{}(e):\n", i));
                                codegen.tab_index += 1;
                                let turn = block.inputs["DEGREES"][1][1].as_str().unwrap();
                                codegen.writeln(&format!("{}.direction += {}", name, turn));
                            }
                            _ => {
                                index += &format!("def block_{}(e):\n", i);
                                index += &format!("\tprint('NOT IMPLEMENTED: {}')\n", block.opcode);
                                index += "\tyield\n";
                                dbg!(&block);
                            }
                        }

                        index += &codegen.data;

                        if let Some(id) = block.next.as_ref() {
                            index += &format!("\tblock = block_list['{}'](e)\n", id);
                            index += "\tif isinstance(block, types.GeneratorType):\n";
                            index += &format!("\t\tyield from block\n");
                        }

                        index += "\treturn\n";
                        index += &format!("block_list['{}'] = block_{}\n", id, i);

                        match block.opcode.as_str() {
                            "event_whenflagclicked" => {
                                index +=
                                    &format!("event_system.on('start', block_list['{}'])\n", id);
                            }
                            _ => (),
                        }
                    }

                    index += &format!("sprite_list.append({name})\n", name = name);
                }

                index += "event_system.fire(Event('start', None))\n";
                index += "screen = pygame.display.set_mode((480, 360))\n";
                index += "done = False\n";
                index += "while not done:\n";
                index += r#"
	for event in pygame.event.get():
		if event.type == pygame.QUIT:
			done = True
        
	for sprite in sprite_list:
		sprite.render(screen)
		
	event_system.update()
	pygame.display.flip()
	screen.fill((255, 255, 255))
"#;
                index += "del midi_player\n";
                index += "pygame.midi.quit()\n";

                let _ = file_creater
                    .write_bytes("index.py", &index.into_bytes())
                    .map_err(|_| ScratchError::Custom("Error Writing index.py".into()))?
                    .mkdir("assets")
                    .is_ok();

                for c in data.targets.iter().flat_map(|t| t.costumes.iter()) {
                    if !file_creater.exists(&c.md5ext) {
                        println!("Downloading {}...", c.md5ext);
                        file_creater
                            .write_bytes(&c.md5ext, &client.get_asset(&c.md5ext)?)
                            .unwrap();
                    }
                }

                for s in data.targets.iter().flat_map(|t| t.sounds.iter()) {
                    if !file_creater.exists(&s.md5ext) {
                        println!("Downloading {}...", s.md5ext);
                        file_creater
                            .write_bytes(&s.md5ext, &client.get_asset(&s.md5ext)?)
                            .unwrap();
                    }
                }
            }
            _ => unimplemented!(),
        }

        Ok(())
    }

    fn run(&mut self, project: &Project) -> ScratchResult<()> {
        let path = project.get_build_path(self);
        let data = make_command()
            .current_dir(&path)
            .arg("python")
            .arg("index.py")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .map_err(|_| ScratchError::Custom("Error Running Command".into()))?;
        dbg!(data.status);
        Ok(())
    }
}
