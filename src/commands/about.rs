use crate::{storage, commands::general, commands::scan, services::color};

// About: list()
// This func is displaying all found 'about'
// Later will be added new func, wich will save 'about' comments into files
// per direcory or just in one file
pub fn list() {
    if !general::is_initialized(){
        general::init_first();
        return; 
    }

    scan::scan();

    let s = storage::load_storage();
    if s.files.len() == 0 || s.files.is_empty() {
        general::nothing_was_found();
        return;
    }

    println!();
    for file in s.files {
        if file.1.abouts.is_empty() {
            continue;
        }

        let mut abouts: Vec<_> = file.1.abouts.iter().collect();
        abouts.sort_by(|a, b| a.index.cmp(&b.index));

        let path = color::paint_str(file.1.path.to_string(), color::Color::Cyan);
        println!(" == {} ==", path);
        for about in abouts {
            if !about.lines.is_empty() {
                let spaces = " ".repeat(about.index.to_string().len()-1);
                let index = color::paint_str(about.index.to_string(), color::Color::Cyan);
                let border = color::paint_str("|".to_string(), color::Color::Cyan);

                let mut first: bool = true;
                for line in &about.lines {
                    if first {
                        let header = color::paint_str(line.header.to_string(), color::Color::Blue);
                        let topic = color::paint_str(line.text.to_string(), color::Color::Yellow);
                        println!(" {} {} {}", index , header, topic);
                        first = false;
                    } else {
                        println!(" {}{} {}",spaces, border, line.text);
                    }
                }
                println!();
            }
        }
    }
}