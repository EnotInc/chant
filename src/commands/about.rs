use::std::fs;
use std::path;
use crate::{storage, commands::general, commands::scan, services::color};

// About: list()
// This func is displaying all found 'about'
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

// About save(output: String)
// This functions is used to save all found 'About' blocks into files
// By default it creates files in every directory, with name "about.md"
// This can be changed by providing different name with flagh -s
// For example: `cahnt about -s readme.md` -> now all 'About' blocks will be saved to 'readme.md'
pub fn save(output: String) {
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

    let mut files: Vec<_> = s.files.iter().collect();
    files.sort_by(|a, b| a.0.cmp(&b.0));

    let mut first = true;
    for file in files {
        let path = path::Path::new(&file.1.dir.to_string()).join(&output);
        let f = path.to_string_lossy().to_string();

        if first {
            let _ = fs::remove_file(&f);
            first = false;
        }

        let is_exisits = fs::exists(&f);
        match is_exisits {
            Ok(v) => {
                if !v { let _ = fs::File::create(&f).unwrap(); }
            }
            Err(_) => { }
        }

        let opened = fs::read_to_string(&f);
        match opened {
            Err(e) => { println!("Can't open file: {}\n{}", &f, e); continue; }
            Ok(v) => { 
                let mut abouts = String::from(v);
                for about in &file.1.abouts {
                    for l in &about.lines {
                        if l.header != ""{
                            abouts.push_str(format!("\n{} {}", l.header, l.text).as_str());
                        } else {
                            abouts.push_str(format!("\n{}", l.text).as_str());
                        }
                    }
                    abouts.push_str("\n");
                }
                let _ = fs::write(&f, abouts);
            }
        }
    }
}