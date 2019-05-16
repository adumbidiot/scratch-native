pub mod api;
mod error;
pub mod target;
pub mod utils;

use crate::scratch_crate::{
    target::Target,
    types::{
        Block,
        ProjectJson,
        SoundJson,
        SpriteJson,
    },
    util::FileCreater,
    ScratchError,
    ScratchResult,
};

/*
pub trait Visitor {
    fn visit<T: Visitable>(&mut self, node: &T);
    fn on_sprite(&mut self, sprite: &SpriteJson);
    fn on_costume(&mut self, costume: &CostumeJson);
}

pub trait Visitable {
    fn visit_with<T: Visitor>(&self, visitor: &mut T);
}
*/

struct PyGameCodeGen {
    tab: usize,
    data: String,
}

impl PyGameCodeGen {
    pub fn new(tab: usize) -> Self {
        PyGameCodeGen {
            tab,
            data: String::new(),
        }
    }

    pub fn write_str(&mut self, data: &str) {
        self.data += data;
    }

    pub fn writeln(&mut self, data: &str) {
        self.data += &"\t".repeat(self.tab);
        self.write_str(&data);
        self.data.push('\n');
    }

    pub fn write_block(&mut self, block: &Block) {
        match block {
            Block::PlaySoundAndWait(sound) => {
                self.writeln(&format!("sound_list['{}'].play()", sound));
            }
            Block::PlayNote(note, beat) => {
                self.writeln(&format!("midi_player.note_on({}, 127)", note));
                self.writeln("t0 = time.time()");
                self.writeln(&format!("while (time.time() - t0) < {}:", beat));
                self.writeln("\tyield");
                self.writeln(&format!("midi_player.note_off({}, 0)", note));
            }
            Block::DoRepeat(n, blocks) => {
                let mut codegen = PyGameCodeGen::new(self.tab);
                codegen.writeln(&format!("for _i in range({}):", n));
                codegen.tab += 1;
                for block in blocks {
                    codegen.write_block(block);
                }
                self.write_str(&codegen.data);
            }
            _ => self.writeln(&format!("#{:?}", block)),
        }
    }
}

struct PyGameIndexFile {
    sprites: Vec<String>,
    sounds: Vec<String>,
}

impl PyGameIndexFile {
    pub fn new() -> Self {
        PyGameIndexFile {
            sprites: Vec::new(),
            sounds: Vec::new(),
        }
    }

    pub fn add_sprite(&mut self, s: &SpriteJson) {
        let mut costume_data = String::new();
        if s.costumes.len() > 0 {
            costume_data = s
                .costumes
                .iter()
                .map(|c| {
                    format!(
                        "{}.costumes.append(Costume({}, {}, {}, 'assets/{}'))\n",
                        s.name, c.center_x, c.center_y, c.resolution, c.src
                    )
                })
                .collect();
        }

        let mut script_data = String::new();

        if let Some(scripts) = &s.scripts {
            for (i, script) in scripts.iter().enumerate() {
                match script.blocks[0] {
                    Block::WhenStart => {
                        let script_name = format!("script_{}_{}", self.sprites.len(), i);
                        let mut codegen = PyGameCodeGen::new(0);

                        codegen.writeln(&format!("def {}(e):", script_name));
                        codegen.tab += 1;
                        for block in script.blocks[1..].iter() {
                            codegen.write_block(&block);
                        }
                        codegen.writeln("return");
                        script_data += &codegen.data;
                        script_data += &format!("event_system.on('start', {})", script_name);
                    }
                    _ => {}
                }
            }
        }

        if let Some(sounds) = &s.sounds {
            for s in sounds {
                self.add_sound(&s);
            }
        }

        let data = format!(
            r#"{name} = Sprite({x}, {y}, {})
{costume}
{script}
sprite_list.append({name})
"#,
            s.current_costume_index,
            costume = costume_data,
            script = script_data,
            name = s.name,
            x = s.x,
            y = s.y,
        );

        self.sprites.push(data);
    }

    pub fn add_sound(&mut self, s: &SoundJson) {
        self.sounds.push(format!(
            "sound_list['{}'] = pygame.mixer.Sound('assets/{}')\n",
            s.name, s.src
        ));
    }

    pub fn build(&self) -> String {
        let mut ret = String::new();
        ret += "import pygame\n";
        ret += "import pygame.midi\n";
        ret += "import time\n";
        ret += "from svg import Parser, Rasterizer\n";

        ret += include_str!("./scratch/event.py");
        ret += include_str!("./scratch/event_dispatcher.py");
        ret += include_str!("./scratch/costume.py");
        ret += include_str!("./scratch/sprite.py");

        ret += "pygame.init()\n";
        ret += "pygame.midi.init()\n";
        ret += "midi_player = pygame.midi.Output(0)\n";
        ret += "midi_player.set_instrument(0)\n";
        ret += "sprite_list = []\n";
        ret += "event_system = EventDispatcher()\n";
        ret += "sound_list = {}\n";

        for sound in self.sounds.iter() {
            ret += sound;
        }

        for s in self.sprites.iter() {
            ret += s;
        }

        ret += "event_system.fire(Event('start', None))\n";
        ret += "screen = pygame.display.set_mode((480, 360))\n";
        ret += "done = False\n";
        ret += r#"
while not done:
	for event in pygame.event.get():
		if event.type == pygame.QUIT:
			done = True
        
	for sprite in sprite_list:
		sprite.render(screen)
		
	event_system.update()
	pygame.display.flip()
	screen.fill((255, 255, 255))

del midi_player
pygame.midi.quit()
"#;
        ret
    }
}
