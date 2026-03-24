use std::{ops::Add};

use crate::{color, storage, commands::general, commands::scan, commands::task};

pub fn list_all() {
    let gt = color::paint_str("Global Tasks".to_string(), color::Color::Blue);
    println!(" ~~ {} ~~",gt);
    task::print_tasks();
}

pub fn list(todo: bool, note: bool, fixme: bool) {
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

    for file in s.files {
        let mut file_data: String = String::new();
        if file.1.comments.is_empty() {
            continue;
        }

        let mut coms: Vec<_> = file.1.comments.iter().collect();
        coms.sort_by(|a, b| a.1.index.cmp(&b.1.index));

        for (_, comment) in coms {
            if !((comment.kind == "TODO" && (any || todo)) ||
                 (comment.kind == "NOTE" && (any || note)) ||
                 (comment.kind == "FIXME" && (any || fixme))) { continue; }

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

            file_data += &format!(" {}: [{}] - {}", index, kind, c.line);

        }
        if !file_data.is_empty() {
            let path = color::paint_str(file.1.path.to_string(), color::Color::Cyan);
                println!();
                println!(" == {} ==", path);
                println!("{}", file_data);

        }
    }
    if is_empty {
        general::nothing_was_found();
        return;
    }
    println!();
}