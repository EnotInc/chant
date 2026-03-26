/// About colors.rs
/// just a small file with [Color] enum, and pub fn [paint_str], wich adds ascii escape sequence to change the color of given line

pub enum Color {
    Reset,
    Red,
    Green,
    Yellow,
    Blue,
    Cyan,
}

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


pub fn paint_str(s: String, c: Color) -> String {
    let ascii = ascii_color(c);
    let r = ascii_color(Color::Reset);
    return format!("{}{}{}", ascii, s, r);
}