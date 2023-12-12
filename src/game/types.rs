#[derive(Debug)]
pub enum Type {
    /// All 18 Types from Pokemon as of 2023
    Normal,
    Fire,
    Water,
    Grass,
    Electric,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dark,
    Dragon,
    Steel,
    Fairy,
}

impl Type {
    pub const TYPES: [&'static Self; 18] = [
        &Self::Normal,
        &Self::Fire,
        &Self::Water,
        &Self::Grass,
        &Self::Electric,
        &Self::Ice,
        &Self::Fighting,
        &Self::Poison,
        &Self::Ground,
        &Self::Flying,
        &Self::Psychic,
        &Self::Bug,
        &Self::Rock,
        &Self::Ghost,
        &Self::Dark,
        &Self::Dragon,
        &Self::Steel,
        &Self::Fairy,
    ];

    pub fn get_random() -> &'static Self {
        Self::TYPES[rand::random::<usize>() % 18]
    }
}
