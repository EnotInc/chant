/// About colors.rs
/// just a small file with [Color] enum, and pub fn [paint_str()], wich adds ascii escape sequence to change the color of given line

/// About |Color|
/// Simple emun, used to work with colors in terminal
pub enum Color {
    Reset,
    Red,
    Green,
    Yellow,
    Blue,
    Cyan,
}

/// About |ascii_color()|
/// used to convert given [Color] to the ascii escape sequence of then color
fn ascii_color(c: Color) -> String {
    match c {
        Color::Reset => { return "\x1b[0m".to_owned() }
        Color::Red => { return "\x1b[31m".to_owned() }
        Color::Green => { return "\x1b[32m".to_owned() }
        Color::Yellow => { return "\x1b[33m".to_owned() }
        Color::Blue => { return "\x1b[34m".to_owned() }
        Color::Cyan => { return "\x1b[36m".to_owned() }
    }
}


/// About |paint_str()|
/// Gets string, [Color], and using [ascii_color()] paint the string
/// At the end added reset ascii escape sequence
pub fn paint_str(s: String, c: Color) -> String {
    let ascii = ascii_color(c);
    let r = ascii_color(Color::Reset);
    return format!("{}{}{}", ascii, s, r);
}