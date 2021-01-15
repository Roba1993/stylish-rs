use stylish_core::{Color, Intensity};

pub(crate) fn color(color: Color) -> &'static str {
    match color {
        Color::Black => "black",
        Color::Red => "red",
        Color::Green => "green",
        Color::Yellow => "yellow",
        Color::Blue => "blue",
        Color::Magenta => "magenta",
        Color::Cyan => "cyan",
        Color::White => "white",
        Color::Default => "inherit",
    }
}

pub(crate) fn intensity(intensity: Intensity) -> &'static str {
    match intensity {
        Intensity::Normal => "inherit",
        Intensity::Bold => "bolder",
        Intensity::Faint => "lighter",
    }
}
