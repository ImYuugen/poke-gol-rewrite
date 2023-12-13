/// Cells are at the core of the game, the simulation's grid is full of them.
use crate::game::types::Type;

/// Cell representation
pub struct Cell {
    /// The type of the cell
    pub type_c: &'static Type,
    /// Base damage dealt to other cells
    pub attack: f32,
    /// Reduces damage taken by this amount
    pub defense: f32,
    /// Decides which cell attacks first, if speed is higher
    /// than that of the opponent, the cell attack first
    pub speed: f32,
    /// Health Points of the cell, when they reach 0,
    /// the cell is destroyed and replaced by an empty one
    pub hp: f32,
}

impl Cell {
    /// Create a cell of specified type
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

/// Generates a 2D Vector of size `size` and fills it with random cells
pub fn init_cells(size: (u32, u32)) -> Vec<Vec<Cell>> {
    (0..size.1)
        .map(|_| {
            (0..size.0)
                .map(|_| Cell::init_random())
                .collect::<Vec<Cell>>()
        })
        .collect()
}
