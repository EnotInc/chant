use std::ops::Add;

use crate::{services::color, storage, commands::general, commands::scan, commands::tasks};

pub fn list_both() {
    if !general::is_initialized() {
        return;
    }

    let gt = color::paint_str("Tasks".to_string(), color::Color::Blue);
    println!("    {}",gt);
    print!(" ~~~~~~~~~~~ ");
    tasks::print_tasks();
}

pub fn list(todo: bool, note: bool, fixme: bool, is_both: bool) {
    if !general::is_initialized(){
        general::init_first();
        return; 
    }

    let any = !todo && !note && !fixme;

    scan::scan();

    let s = storage::load_storage();
    let mut is_empty: bool = true;
    if s.files.len() == 0 || s.files.is_empty() {
        general::nothing_was_found();
        return;
    }

    let mut files: Vec<_> = s.files.iter().collect(); 
    files.sort_by(|a, b| b.1.path.cmp(&a.1.path));

    for file in files {
        let mut file_data: String = String::new();

        if file.1.comments.is_empty() { continue; }

        let mut coms: Vec<_> = file.1.comments.iter().collect();
        coms.sort_by(|a, b| a.1.index.cmp(&b.1.index));

        for (_, comment) in coms {
            match comment.kind.as_str() {
                 "TODO" => { if  !todo && !any { continue; } }
                 "NOTE" => { if  !note && !any { continue; } }
                "FIXME" => { if !fixme && !any { continue; } }
                _ => {}
            };


            is_empty = false;
            let c: storage::Comment = comment.clone();

            let mut kind_color = color::Color::Yellow;
            match c.kind.as_str() {
                 "TODO" => kind_color = color::Color::Blue,
                 "NOTE" => kind_color = color::Color::Green,
                "FIXME" => kind_color = color::Color::Red,
                _ => {}
            }
            let kind = color::paint_str(c.kind, kind_color);
            let index = color::paint_str(c.index.add(1).to_string(), color::Color::Cyan);
            let line: String;
            match c.kept {
                true => { line = color::paint_str(c.line, color::Color::Code); }
                false => { line = c.line }
            }

            file_data += &format!(" {}: [{}] - {}\n", index, kind, line);

        }
        if !file_data.is_empty() {
            let path = color::paint_str(file.1.path.to_string(), color::Color::Cyan);
            println!();
            println!(" == {} ==", path);
            print!("{}", file_data);
        }
    }

    if is_empty {
        general::nothing_was_found();
    } else { // just to make output prettier
        println!()
    }

    if is_both {
        list_both();
        return;
    }

    if !s.tasks.is_empty()  {
        let t = color::paint_str("~~~".to_string(), color::Color::Blue);
        println!(" {}", t);
    }
}