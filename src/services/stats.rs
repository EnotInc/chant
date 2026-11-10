use crate::{
    services::color,
    storage::{Stats, Status, Storage, Summary, new_stats}
};

pub fn get_stats(s: &mut Storage) -> Stats {
    let mut todos: u8 = 0;
    let mut notes: u8 = 0;
    let mut fixmes: u8 = 0;

    let mut backlog: u8 = 0;
    let mut in_progress: u8 = 0;
    let mut done: u8 = 0;

    // Long live O(n²)!
    for file in s.files.iter() {
        for c in file.1.comments.iter() {
            match c.1.kind.as_str() {
                "TODO" => {todos += 1}
                "NOTE" => {notes += 1}
                "FIXME" => {fixmes += 1}
                _ => {}
            }
        }
    }

    for task in s.tasks.iter() {
        match task.1.status {
            Status::Backlog => {backlog += 1}
            Status::InProgress => {in_progress += 1}
            Status::Done => {done += 1}
        }
    }

    let mut stats = new_stats();

    stats.todos = todos;
    stats.notes = notes;
    stats.fixmes = fixmes;

    stats.backlog = backlog;
    stats.in_progress = in_progress;
    stats.done = done;

    return stats;
}

pub fn stats_to_string(stats: Option<Stats>) -> String {
    match stats {
        None => {
            let r = color::paint_str("-".to_string(), color::Color::Red);
            let b = color::paint_str("-".to_string(), color::Color::Blue);
            let c = color::paint_str("-".to_string(), color::Color::Cyan);
            let g = color::paint_str("-".to_string(), color::Color::Green);

            //return format!("[ {b} {g} {r} ] [ {b} {c} {g} ]");
            return format!("[ {b} {g} {r} | {b} {c} {g} ]");
        }
        Some(s) => {
            let t = color::paint_str(format!("{}", s.todos).to_string(), color::Color::Blue);
            let n = color::paint_str(format!("{}", s.notes).to_string(), color::Color::Green);
            let f = color::paint_str(format!("{}", s.fixmes).to_string(), color::Color::Red);

            let b = color::paint_str(format!("{}", s.backlog).to_string(), color::Color::Blue);
            let i = color::paint_str(format!("{}", s.in_progress).to_string(), color::Color::Cyan);
            let d = color::paint_str(format!("{}", s.done).to_string(), color::Color::Green);

            //return format!("[ {t} {n} {f} ] [ {b} {i} {d} ]");
            return format!("[ {t} {n} {f} | {b} {i} {d} ]");
        }
    }
}

pub fn print_summary(summary: &Summary) {
    let f_v = color::paint_str("│".to_string(), color::Color::Red);
    let f_c = color::paint_str("└──".to_string(), color::Color::Red);

    let n_v = color::paint_str("│".to_string(), color::Color::Green);
    let n_c = color::paint_str("└──".to_string(), color::Color::Green);

    let t_v = color::paint_str("│".to_string(), color::Color::Blue);
    let t_c = color::paint_str("└──".to_string(), color::Color::Blue);

    let todos = color::paint_str("TODO".to_string(), color::Color::Blue);
    let notes = color::paint_str("NOTE".to_string(), color::Color::Green);
    let fixmes = color::paint_str("FIXME".to_string(), color::Color::Red);

    println!("   {t_v} {n_v} {f_v}");
    println!("   {t_v} {n_v} {f_c} {fixmes}: {}", summary.fixmes);
    println!("   {t_v} {n_c} {notes}: {}", summary.notes);
    println!("   {t_c} {todos}: {}", summary.todos);
    println!()
}