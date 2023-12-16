/// Cells are at the core of the game, the simulation's grid is full of them.
use crate::game::types::Type;

/// Cell representation
#[derive(Clone, Copy)]
pub struct Cell {
    /// The type of the cell
    pub type_c: Option<&'static Type>,
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
    fn recv_damage(&mut self, dmg: f32) -> bool {
        self.hp -= dmg;
        if self.hp <= 0.0 {
            self.type_c = None;
            true
        } else {
            false
        }
    }

    /// Create cell of random type
    fn init_random() -> Self {
        Self {
            type_c: Some(Type::get_random()),
            ..Default::default()
        }
    }

    pub async fn attack_neighbours(cells: &mut [Vec<Self>], c1: (usize, usize)) {
        let (min_x, max_x) = (
            if c1.0 == 0 { 0 } else { 1 },
            if c1.0 == cells.len() - 1 { 1 } else { 2 }
        );
        let (min_y, max_y) = (
            if c1.1 == 0 { 0 } else { 1 },
            if c1.1 == cells[0].len() - 1 { 1 } else { 2 }
        );

        for off_x in min_x..max_x {
            for off_y in min_y..max_y {
                Self::attack(cells, cells[c1.0][c1.1], cells[c1.0 - min_x + off_x][c1.1 - min_y + off_y]);
            }
        }
    }

    fn attack(cells: &mut [Vec<Self>], c1: Cell, c2: Cell) {
        let (mut first, mut second) = if c1.speed > c2.speed { (c1, c2) } else { (c2, c1) };
        if let (Some(t1), Some(t2)) = (c1.type_c, c2.type_c) {
            first.recv_damage(second.attack * t2.get_multiplier(t1) - first.defense);
            second.recv_damage(first.attack * t2.get_multiplier(t1) - second.defense);
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            type_c: None,
            attack: 5.0,
            defense: 5.0,
            speed: 5.0,
            hp: 10.0,
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
