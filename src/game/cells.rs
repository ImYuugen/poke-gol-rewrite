/// Cells are at the core of the game, the simulation's grid is full of them.
use crate::game::types::Type;

/// Cell representation
#[derive(Clone, Copy)]
pub struct Cell {
    /// The type of the cell
    pub type_c: Type,
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
    pub changed: bool,
}

impl Cell {
    /// Create cell of random type
    fn init_random() -> Self {
        Self {
            type_c: Type::get_random(),
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

        for off_x in 0..=max_x {
            for off_y in 0..=max_y {
                Self::attack(cells, (c1.0, c1.1), (c1.0 - min_x + off_x, c1.1 - min_y + off_y));
            }
        }
    }

    fn attack(cells: &mut [Vec<Self>], c1: (usize, usize), c2: (usize, usize)) {
        let (mut cell1, mut cell2) = (cells[c1.0][c1.1], cells[c2.0][c2.1]);
        if cell1.changed || cell2.changed {
            return;
        }
        if cell1.speed >= cell2.speed {
            if cell2.recv_damage(cell1.attack * cell1.type_c.get_multiplier(cell2.type_c) - cell2.defense) {
                cell2.type_c = cell1.type_c;
            }
            cells[c2.0][c2.1] = cell2;
        } else {
            if cell1.recv_damage(cell2.attack * cell2.type_c.get_multiplier(cell1.type_c) - cell1.defense) {
                cell1.type_c = cell2.type_c;
            }
            cells[c1.0][c1.1] = cell1;
        }
    }

    fn recv_damage(&mut self, dmg: f32) -> bool {
        let dmg = dmg.max(0.0);
        self.hp -= dmg;
        if self.hp <= 0.0 {
            // TODO: Mutate cell
            self.hp = 25.0;
            true
        } else {
            false
        }
    }
}

impl From<Type> for Cell {
    fn from(value: Type) -> Self {
        Self {
            type_c: value,
            ..Default::default()
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            type_c: Type::Normal,
            attack: 5.0,
            defense: 0.0,
            speed: 5.0,
            hp: 25.0,
            changed: false,
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
