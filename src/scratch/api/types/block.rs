use serde::{
    de::{
        SeqAccess,
        Visitor,
    },
    Deserialize,
    Deserializer,
    Serialize,
};

#[derive(Serialize, Debug)]
pub enum Block {
    WhenStart,
    PlaySoundAndWait(PlaySoundAndWait),
    Custom { arr: Vec<String> },
    Unknown { arr: Vec<String> },
}

impl Block {
    pub fn get_name(&self) -> &'static str {
        match self {
            Block::WhenStart => "WhenStart",
            Block::PlaySoundAndWait(_) => "PlaySoundAndWait",
            Block::Custom { .. } => "Custom",
            Block::Unknown { .. } => "Unknown",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaySoundAndWait {
    pub song: String,
}

impl<'d> Deserialize<'d> for Block {
    fn deserialize<D>(deserializer: D) -> Result<Block, D::Error>
    where
        D: Deserializer<'d>,
    {
        deserializer.deserialize_any(BlockVisitor)
    }
}

struct BlockVisitor;

impl<'de> Visitor<'de> for BlockVisitor {
    type Value = Block;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Block Enum or Array")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut elements: Vec<String> = Vec::new();
        while let Some(el) = seq.next_element()? {
            elements.push(el);
        }

        return match elements
            .get(0)
            .ok_or(serde::de::Error::invalid_length(0, &self))?
            .as_str()
        {
            "whenGreenFlag" => Ok(Block::WhenStart),
            "doPlaySoundAndWait" => {
                let mut music = String::new();
                std::mem::swap(
                    &mut music,
                    elements
                        .get_mut(1)
                        .ok_or(serde::de::Error::invalid_length(1, &self))?,
                );
                Ok(Block::PlaySoundAndWait(PlaySoundAndWait { song: music }))
            }
            _ => Ok(Block::Unknown { arr: elements }),
        };
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "WhenStart" => {
                return Ok(Block::WhenStart);
            }
            _ => {
                return Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(v),
                    &self,
                ));
            }
        }
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let key: &str = map
            .next_key()?
            .ok_or(serde::de::Error::invalid_length(0, &self))?;
        match key {
            "PlaySoundAndWait" => {
                let block: PlaySoundAndWait = map
                    .next_value::<Option<PlaySoundAndWait>>()?
                    .ok_or(serde::de::Error::invalid_length(0, &self))?;
                return Ok(Block::PlaySoundAndWait(block));
            }
            _ => {
                return Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(key),
                    &self,
                ))
            }
        }
        return Ok(Block::WhenStart);
    }
}
