use crate::game::{cells, types};
use crossterm::{cursor, execute};
use owo_colors::{FgDynColorDisplay, OwoColorize, Rgb};

pub fn term_init() {
    let _ = execute!(std::io::stdout(), cursor::Hide);
}

pub fn term_restore() {
    let _ = execute!(std::io::stdout(), cursor::Show);
}

/// Prints every cell on the board.
pub fn draw(cells: &[Vec<cells::Cell>]) -> Result<(), std::io::Error> {
    let mut buf = String::with_capacity(cells.len() * cells[0].len());

    for row in cells.iter() {
        for cell in row.iter() {
            buf.push_str(&format!("{}", type_repr(cell.type_c)));
        }
        buf.push('\n');
    }

    print!("{}", buf);
    execute!(std::io::stdout(), cursor::MoveUp(cells.len() as u16))?;
    Ok(())
}

/// Returns the character that should represent a cell of a given [Type](game/Type).
fn type_repr(t: Option<types::Type>) -> FgDynColorDisplay<'static, Rgb, &'static str> {
    let (r, g, b) = match t {
        Some(types::Type::Normal) => (168, 167, 122),
        Some(types::Type::Fire) => (238, 129, 48),
        Some(types::Type::Water) => (99, 144, 240),
        Some(types::Type::Grass) => (122, 199, 76),
        Some(types::Type::Electric) => (247, 208, 44),
        Some(types::Type::Ice) => (150, 217, 214),
        Some(types::Type::Fighting) => (194, 46, 40),
        Some(types::Type::Poison) => (163, 62, 161),
        Some(types::Type::Ground) => (226, 191, 101),
        Some(types::Type::Flying) => (169, 143, 243),
        Some(types::Type::Psychic) => (249, 85, 135),
        Some(types::Type::Bug) => (166, 185, 26),
        Some(types::Type::Rock) => (182, 161, 54),
        Some(types::Type::Ghost) => (115, 87, 151),
        Some(types::Type::Dark) => (112, 87, 70),
        Some(types::Type::Dragon) => (111, 53, 252),
        Some(types::Type::Steel) => (183, 183, 206),
        Some(types::Type::Fairy) => (214, 133, 173),
        None => (0, 0, 0),
    };
    "■".truecolor(r, g, b)
}
