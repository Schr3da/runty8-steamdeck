use crate::editor::serialize::Serialize;

use super::sprite_sheet::Sprite;
use itertools::Itertools;

type SpriteId = u8;

#[derive(Debug, Clone)]
pub struct Map {
    // Don't really want the size to change
    pub(crate) map: [SpriteId; Self::MAP_SIZE],
}

impl Map {
    const SCREEN_SIZE_PIXELS: usize = 128;
    const SCREENS_WIDTH: usize = 8; // map is 8 screens wide
    const SCREENS_HEIGHT: usize = 4; // map is 4 screens tall (actually 2, bottom 2 are shared with spritesheet)

    const SPRITES_PER_SCREEN_ROW: usize = Self::SCREEN_SIZE_PIXELS / Sprite::WIDTH;
    pub const WIDTH_SPRITES: usize = Self::SCREENS_WIDTH * Self::SPRITES_PER_SCREEN_ROW;
    pub const HEIGHT_SPRITES: usize = Self::SCREENS_HEIGHT * Self::SPRITES_PER_SCREEN_ROW;
    const MAP_SIZE: usize = Self::WIDTH_SPRITES * Self::HEIGHT_SPRITES;

    // TODO: Make pub(crate)
    pub fn new() -> Self {
        let mut map = [0; Self::MAP_SIZE];

        map[0] = 1;
        map[1] = 1;
        map[2] = 1;

        Map { map }
    }

    pub fn from_slice(v: &[SpriteId]) -> Self {
        let mut m = Self::new();

        for (i, s) in v.iter().copied().enumerate() {
            m.map[i] = s;
        }

        m
    }

    pub(crate) fn mget(&self, cel_x: usize, cel_y: usize) -> u8 {
        let index = cel_x + cel_y * Map::WIDTH_SPRITES;
        // TODO: Handle like pico8
        assert!(index <= self.map.len());

        self.map[index]
    }

    pub(crate) fn mset(&mut self, cel_x: usize, cel_y: usize, sprite: u8) {
        let index = cel_x + cel_y * Map::WIDTH_SPRITES;
        // TODO: Handle like pico8
        assert!(index <= self.map.len());

        self.map[index] = sprite;
    }

    pub fn iter(&self) -> impl Iterator<Item = SpriteId> + '_ {
        self.map.iter().copied()
    }
}

impl Map {
    // TODO: Make sure this works
    pub(crate) fn deserialize(str: &str) -> Result<Self, String> {
        let map: [SpriteId; Self::MAP_SIZE] = str
            .as_bytes()
            .iter()
            .copied()
            .filter_map(|c| {
                let c = c as char;

                c.to_digit(16).map(|c| c as u8)
            })
            .tuples()
            .map(|(high, low)| high << 4 | low)
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|error: Vec<u8>| format!("Error deserializing map {}", error.len()))?;

        Ok(Self { map })
    }
}

impl Serialize for Map {
    fn file_name() -> String {
        "map.txt".to_owned()
    }

    // TODO: Make sure this works
    fn serialize(&self) -> String {
        self.map
            .iter()
            .chunks(Map::WIDTH_SPRITES)
            .into_iter()
            .map(|chunk| chunk.map(|n| format!("{:0>2X}", n)).join(" "))
            .join("\n")
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}
