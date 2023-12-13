use crate::game::{cells, types};
use crossterm::{cursor, execute};
use owo_colors::{FgDynColorDisplay, OwoColorize, Rgb};

/// Prints every cell on the board.
pub fn draw(cells: &[Vec<cells::Cell>]) -> Result<(), std::io::Error> {
    let mut buf = String::with_capacity(cells.len() * cells[0].len());

    execute!(std::io::stdout(), cursor::SavePosition)?;
    for row in cells.iter() {
        for cell in row.iter() {
            buf.push_str(&format!("{}", type_repr(cell.type_c)));
        }
        buf.push('\n');
    }

    // FIX: Board not overwriting when console not cleared
    print!("{}", buf);
    execute!(std::io::stdout(), cursor::RestorePosition)?;
    Ok(())
}

/// Returns the character that should represent a cell of a given [Type](game/Type).
fn type_repr(t: &'static types::Type) -> FgDynColorDisplay<'_, Rgb, &str> {
    match *t {
        types::Type::Normal => "N".truecolor(168, 167, 122),
        types::Type::Fire => "F".truecolor(238, 129, 48),
        types::Type::Water => "W".truecolor(99, 144, 240),
        types::Type::Grass => "G".truecolor(122, 199, 76),
        types::Type::Electric => "E".truecolor(247, 208, 44),
        types::Type::Ice => "I".truecolor(150, 217, 214),
        types::Type::Fighting => "F".truecolor(194, 46, 40),
        types::Type::Poison => "P".truecolor(163, 62, 161),
        types::Type::Ground => "G".truecolor(226, 191, 101),
        types::Type::Flying => "F".truecolor(169, 143, 243),
        types::Type::Psychic => "P".truecolor(249, 85, 135),
        types::Type::Bug => "B".truecolor(166, 185, 26),
        types::Type::Rock => "R".truecolor(182, 161, 54),
        types::Type::Ghost => "G".truecolor(115, 87, 151),
        types::Type::Dark => "D".truecolor(112, 87, 70),
        types::Type::Dragon => "D".truecolor(111, 53, 252),
        types::Type::Steel => "S".truecolor(183, 183, 206),
        types::Type::Fairy => "F".truecolor(214, 133, 173),
    }
}
