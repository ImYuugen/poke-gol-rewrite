/// Cells are at the core of the game, the simulation's grid is full of them.

use crate::game::types::Type;

pub struct Cell {
    pub type_c: &'static Type,
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub hp: f32,
}

impl Cell {
    fn init_type(t: &'static Type) -> Self {
        Self {
            type_c: t,
            ..Default::default()
        }
    }

    /// Create cell of random type
    fn init_random() -> Self {
        Self {
            type_c: Type::get_random(),
            ..Default::default()
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            type_c: &Type::Normal,
            attack: 2.5,
            defense: 2.5,
            speed: 2.5,
            hp: 2.5,
        }
    }
}

pub fn init_cells(size: (u32, u32)) -> Vec<Vec<Cell>> {
    (0..size.1)
        .map(|_| (0..size.0)
            .map(|_| Cell::init_random())
            .collect::<Vec<Cell>>())
        .collect()
}
