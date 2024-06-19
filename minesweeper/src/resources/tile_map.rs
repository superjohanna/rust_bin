use bevy::prelude::*;
use rand::{prelude::SliceRandom, thread_rng};

use crate::components::coordinates::CoordinateU16;

use super::tile::Tile;

#[derive(Debug, Clone)]
pub struct TileMap {
    bomb_count: u16,
    height: u16,
    width: u16,
    map: Vec<Tile>,
}

impl TileMap {
    pub fn empty(width: u16, height: u16) -> Self {
        let map = (0..height * width).map(|_| Tile::Empty).collect();

        Self {
            bomb_count: 0,
            height,
            width,
            map,
        }
    }

    pub fn spread_bombs(&mut self, bomb_count: u16) {
        self.bomb_count = bomb_count;
        let mut map_pointers: Vec<&mut Tile> = self.map.iter_mut().collect();
        let mut rng = thread_rng();

        map_pointers.shuffle(&mut rng);
        map_pointers
            .into_iter()
            .take(self.bomb_count as usize)
            .for_each(|x| *x = Tile::Bomb)
    }

    pub fn bomb_count_at(&self, coordinate: CoordinateU16) -> u8 {
        self.neighbours(coordinate).filter(|x| x.is_bomb()).count() as u8
    }

    pub fn bomb_count_at_index(&self, index: usize) -> u8 {
        let coordinate = CoordinateU16 {
            x: index as u16 % self.width,
            y: index as u16 / self.width,
        };
        self.bomb_count_at(coordinate)
    }

    pub fn neighbour_coordinates(
        &self,
        coordinate: CoordinateU16,
    ) -> impl Iterator<Item = CoordinateU16> + '_ {
        SQUARE_NEIGHBOUR_COORDINATES
            .iter()
            .copied()
            .map(move |tuple| coordinate + tuple)
            .filter(|coordinate| coordinate.x < self.width && coordinate.y < self.height)
    }

    pub fn true_neighbour_coordinates(
        &self,
        coordinate: CoordinateU16,
    ) -> impl Iterator<Item = CoordinateU16> + '_ {
        TRUE_SQUARE_NEIGHBOUR_COORDINATES
            .iter()
            .copied()
            .map(move |tuple| coordinate + tuple)
            .filter(|coordinate| coordinate.x < self.width && coordinate.y < self.height)
    }

    pub fn neighbours(&self, coordinate: CoordinateU16) -> impl Iterator<Item = Tile> + '_ {
        self.neighbour_coordinates(coordinate).map(|x| self[x])
    }

    pub fn height(&self) -> &u16 {
        &self.height
    }

    pub fn width(&self) -> &u16 {
        &self.width
    }

    pub fn bomb_count(&self) -> &u16 {
        &self.bomb_count
    }
}

impl std::ops::Index<(u16, u16)> for TileMap {
    type Output = Tile;

    fn index(&self, (x, y): (u16, u16)) -> &Self::Output {
        &self.map[x as usize + (y * self.width) as usize]
    }
}

impl std::ops::Index<CoordinateU16> for TileMap {
    type Output = Tile;

    fn index(&self, coordinate: CoordinateU16) -> &Self::Output {
        &self.map[coordinate.x as usize + (coordinate.y * self.width) as usize]
    }
}

impl std::ops::Deref for TileMap {
    type Target = Vec<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl std::ops::DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

//#[cfg(feature = "debug")]
impl std::fmt::Display for TileMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = format!(
            "Map ({};{}) with {} bombs:\n",
            self.width, self.height, self.bomb_count
        );

        buffer += "|";

        buffer += &(0..self.width).map(|_| "-").collect::<String>();

        buffer += &self
            .map
            .iter()
            .enumerate()
            .map(|(index, element)| {
                let mut buf = String::new();
                if index % self.width as usize == 0 {
                    buf += "|\n|"
                }
                buf += &match element {
                    Tile::Bomb => "*".to_string(),
                    Tile::Empty => match self.bomb_count_at_index(index) {
                        0 => " ".to_string(),
                        x => x.to_string(),
                    },
                };
                buf
            })
            .collect::<String>();
        buffer += "|\n|";
        buffer += &(0..self.width).map(|_| "-").collect::<String>();
        buffer += "|";

        write!(f, "{}", buffer)
    }
}

const SQUARE_NEIGHBOUR_COORDINATES: [(i8, i8); 8] = [
    // Bottom left
    (-1, -1),
    // Bottom
    (0, -1),
    // Bottom right
    (1, -1),
    // Left
    (-1, 0),
    // Right
    (1, 0),
    // Top Left
    (-1, 1),
    // Top
    (0, 1),
    // Top right
    (1, 1),
];

const TRUE_SQUARE_NEIGHBOUR_COORDINATES: [(i8, i8); 4] = [
    // Bottom
    (0, -1),
    // Left
    (-1, 0),
    // Right
    (1, 0),
    // Top
    (0, 1),
];
