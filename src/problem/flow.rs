use std::hash::Hash;

pub struct Problem<T: PartialEq + Eq + Hash + Clone> {
    start_state: Board<T>,
}

impl<T: PartialEq + Eq + Hash + Clone> Problem<T> {
    pub fn new(start_state: Board<T>) -> Self {
        Self { start_state }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Board<T: PartialEq + Eq + Hash + Clone> {
    tiles: Vec<Tile<T>>,
    width: usize,
    height: usize,
}

impl<T: PartialEq + Eq + Hash + Clone> Board<T> {
    pub fn new(tiles: Vec<Tile<T>>, width: usize, height: usize) -> Result<Self, InvalidState> {
        width
            .checked_mul(height)
            .filter(|&size| tiles.len() == size)
            .map(|_| Self {
                tiles,
                width,
                height,
            })
            .ok_or(InvalidState)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Tile<T: PartialEq + Eq + Hash + Clone> {
    Empty,
    Filled(T),
}

#[derive(Debug, thiserror::Error)]
pub struct InvalidState;

impl std::fmt::Display for InvalidState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid state configuration")
    }
}

#[macro_export]
macro_rules! board {
    ([$([$($t:tt)*])+]) => { {
        let mut v = Vec::new();
        let mut w = 0usize;
        let mut h = 0usize;
        $(
            w = 0;
            h += 1;
            $(
                v.push($crate::problem::flow::board!(@tile $t));
                w += 1;
            )*
        )*
        $crate::problem::flow::Board::new(v, w, h).unwrap()
    } };
    (@tile .) => {
        $crate::problem::flow::Tile::Empty
    };
    (@tile $e:expr) => {
        $crate::problem::flow::Tile::Filled($e)
    };
}

pub use board;
