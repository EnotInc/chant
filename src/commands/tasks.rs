use regex::Regex;
use std::{collections::HashMap, fs, io::{self, Write}};

use crate::{commands::general, services::color, storage::{self, Status}};

pub fn add_task(text: String) {
    if !general::is_initialized() {
        general::init_first();
        return;
    }

    let mut s = storage::load_storage();
    let t = storage::new_task(text);
    let id = &t.id;

    if s.tasks.contains_key(id) {
        println!("This taks already exists");
        return;
    }

    s.tasks.insert(id.to_string(), t);
    storage::save_storage(&s);
}

pub fn remove_task(ids: Vec<String>){
    if !general::is_initialized() {
        general::init_first();
        return;
    }

    let mut s = storage::load_storage();
    for id in ids {
        if !s.tasks.contains_key(&id) {
            println!("Task with this id doesn't exists");
            continue;
        }
        s.tasks.remove(&id);

        let colored_id = color::paint_str(id, color::Color::Yellow);
        println!("Task {} was removed", colored_id);
    }

    storage::save_storage(&s);
}

pub fn remove_all() {
    if !general::is_initialized() {
        general::init_first();
        return;
    }

    let mut s = storage::load_storage();
    s.tasks = HashMap::new();
    println!("All tasks was removed");

    storage::save_storage(&s);
}

pub fn remove_done() {
    if !general::is_initialized() {
        general::init_first();
        return;
    }

    let mut s = storage::load_storage();
    for task in s.tasks.clone() {
        if task.1.status == Status::Done {
            s.tasks.remove(&task.0);
        }
    }

    storage::save_storage(&s);
}

pub fn edit_task(id: String) {
    if !general::is_initialized() {
        general::init_first();
        return;
    }

    let mut s = storage::load_storage();
    if !s.tasks.contains_key(&id) {
        println!("Task with this id doesn't exists");
        return;
    }

    let point = color::paint_str("> ".to_string(), color::Color::Blue);
    let idstr = color::paint_str(id.to_string(), color::Color::Yellow);
    print!(" Enter new message for task {}:\n {}", idstr, point);

    let mut new_message: String = String::new();
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut new_message).expect("Faailed to read new message");

    let task = s.tasks.get_mut(&id).unwrap();
    task.text = new_message.trim().to_string();

    storage::save_storage(&s);
}

pub fn done_task(id: String) {
    if !general::is_initialized() {
        general::init_first();
        return;
    }

    let mut s = storage::load_storage();
    if !s.tasks.contains_key(&id) {
        println!("Task with this id doesn't exists");
        return;
    }

    let task = s.tasks.get_mut(&id).unwrap();
    task.status = Status::Done;

    storage::save_storage(&s);
    println!("Status changed");
}

pub fn progress_task(id: String) {
    if !general::is_initialized() {
        general::init_first();
        return;
    }

    let mut s = storage::load_storage();
    if !s.tasks.contains_key(&id) {
        println!("Task with this id doesn't exists");
        return;
    }

    let task = s.tasks.get_mut(&id).unwrap();
    task.status = Status::InProgress;

    storage::save_storage(&s);
    println!("Status changed");
}

pub fn backlog_task(id: String) {
    if !general::is_initialized() {
        general::init_first();
        return;
    }

    let mut s = storage::load_storage();
    if !s.tasks.contains_key(&id) {
        println!("Task with this id doesn't exists");
        return;
    }

    let task = s.tasks.get_mut(&id).unwrap();
    task.status = Status::Backlog;

    storage::save_storage(&s);
    println!("Status changed");
}

pub fn migrate() {
    let _true = Regex::new(r"true").unwrap();
    let _false = Regex::new(r"false").unwrap();
    let _done = Regex::new(r"done").unwrap();

    // TODO: move to storage as "get_path();"
    let path = "./.chant/storage.json"; 
    let read = fs::read_to_string(path);
    match read {
        Ok(v) => {
            let mut new_sotrage = _true.replace_all(&v, "\"Done\"").to_string();
            new_sotrage = _false.replace_all(&new_sotrage, "\"Backlog\"").to_string();
            new_sotrage = _done.replace_all(&new_sotrage, "status").to_string();

            let write = fs::write(path, &new_sotrage);
            match write {
                Ok(_) => { println!("Migration ended"); }
                Err(e) => { println!("Unable to migrate old tasks due to error:\n{}", e); }
            }
            return;
        }
        Err(e) => {
            println!("Unable to migrate old task due to error:\n{}", e);
            return;
        }
    }
}

// TODO: update visualisation
pub fn print_tasks() {
    if !general::is_initialized() {
        general::init_first();
        return;
    }

    let s = storage::load_storage();
    let mut tasks: Vec<_> = s.tasks.values().collect();
    tasks.sort_by_key(|task| &task.status);

    let mut statuses: Vec<Status> = Vec::new();
    for task in tasks {
        let header: String;
        match task.status {
            Status::Backlog => {
                if !statuses.contains(&Status::Backlog) {
                    header = color::paint_str("Backlog".to_string(), color::Color::Blue);
                    println!("{}", header);
                }
                statuses.push(Status::Backlog);
            }
            Status::InProgress => {
                if !statuses.contains(&Status::InProgress) {
                    header = color::paint_str("InProgress".to_string(), color::Color::Cyan);
                    println!("\n{}", header);
                }
                statuses.push(Status::InProgress);
            }
            Status::Done => {
                if !statuses.contains(&Status::Done) {
                    header = color::paint_str("Done".to_string(), color::Color::Green);
                    println!("\n{}", header);
                }
                statuses.push(Status::Done);
            }
        }

        let idstr = color::paint_str(task.id.to_string(), color::Color::Yellow);
        println!(" {} {}", idstr, task.text);
    }
    println!();
}