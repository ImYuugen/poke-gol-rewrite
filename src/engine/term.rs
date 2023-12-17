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
fn type_repr(t: types::Type) -> FgDynColorDisplay<'static, Rgb, &'static str> {
    let (r, g, b) = match t {
        types::Type::Normal => (168, 167, 122),
        types::Type::Fire => (238, 129, 48),
        types::Type::Water => (99, 144, 240),
        types::Type::Grass => (122, 199, 76),
        types::Type::Electric => (247, 208, 44),
        types::Type::Ice => (150, 217, 214),
        types::Type::Fighting => (194, 46, 40),
        types::Type::Poison => (163, 62, 161),
        types::Type::Ground => (226, 191, 101),
        types::Type::Flying => (169, 143, 243),
        types::Type::Psychic => (249, 85, 135),
        types::Type::Bug => (166, 185, 26),
        types::Type::Rock => (182, 161, 54),
        types::Type::Ghost => (115, 87, 151),
        types::Type::Dark => (112, 87, 70),
        types::Type::Dragon => (111, 53, 252),
        types::Type::Steel => (183, 183, 206),
        types::Type::Fairy => (214, 133, 173),
    };
    "■".truecolor(r, g, b)
}
