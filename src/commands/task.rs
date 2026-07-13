use std::{collections::HashMap, io::{self, Write}};

use crate::{storage, services::color, commands::general};

/// So I've wanned to be able to keep tack of my small chanes, and tasks inside of any project, so I added tasks
/// It's basically a small 'todo' app inside of 'chant', but it's quite good, and even useful
/// Tasks are stored in `.chant/storage.json` file, and now they are local. Maybe later, if I'll add a global config, I'll also implement a globals storage, so I can save all tasks, there. Idrk yet

pub fn add_task(text: String) {
    if !general::is_initialized() {
        general::init_first();
        return;
    }

    let mut s = storage::load_storage();
    let t = storage::new_task(text);
    let id = &t.id;

    if s.tasks.contains_key(&t.id) {
        println!("This task already exists");
        return;
    }

    let color_id = color::paint_str(t.id.clone(), color::Color::Yellow);
    let color_done = color::paint_str("[ ]".to_string(), color::Color::Red);
    println!(" {} {} {}\n", color_id, color_done, t.text);

    s.tasks.insert(id.to_string(), t);

    storage::save_storage(&s);
}

pub fn remove_task(ids: Vec<String>) {
    if !general::is_initialized() {
        general::init_first();
        return;
    }
    let mut s = storage::load_storage();
    for id in ids {
        if !s.tasks.contains_key(&id) {
            println!("task with this id doesn't exists");
            return;
        }
        s.tasks.remove(&id);

        let color_id = color::paint_str(id, color::Color::Yellow);
        println!("Task {} was removed", color_id);
    }
    println!();

    storage::save_storage(&s);
}

pub fn remove_all() {
    if !general::is_initialized() {
        general::init_first();
        return;
    }
    let mut s = storage::load_storage();
    s.tasks = HashMap::new();
    println!("All tasks was removed\n");

    storage::save_storage(&s);
}

pub fn remove_done() {
    if !general::is_initialized() {
        general::init_first();
        return;
    }
    let mut s = storage::load_storage();
    for task in s.tasks.clone() {
        if task.1.done {
            s.tasks.remove(&task.0);
        }
    }
    println!("Completed tasks was removed\n");

    storage::save_storage(&s);
}

pub fn done_task(id: String) {
    if !general::is_initialized() {
        general::init_first();
        return;
    }
    let mut s = storage::load_storage();
    if !s.tasks.contains_key(&id) {
        println!("task with this id doesn't exists");
        return;
    }

    let task = s.tasks.get_mut(&id).unwrap();
    task.done = true;

    let color_id = color::paint_str(id, color::Color::Yellow);
    let color_done = color::paint_str("[x]".to_string(), color::Color::Green);
    println!(" {} {} {}\n", color_id, color_done, task.text);

    storage::save_storage(&s);
}

pub fn edit_task(id: String) {
    if !general::is_initialized() {
        general::init_first();
        return;
    }

    let mut new_message: String = String::new();

    let mut s = storage::load_storage();
    if !s.tasks.contains_key(&id) {
        println!("task with this id doesn't exists");
        return;
    }

    let task = s.tasks.get_mut(&id).unwrap();

    let id = color::paint_str(task.id.to_string(), color::Color::Yellow);
    let point = color::paint_str("> ".to_string(), color::Color::Blue);
    let mut done: String;
    match task.done {
        false => { done = "[ ]".to_string(); done = color::paint_str(done, color::Color::Red); }
        true => { done = "[x]".to_string(); done = color::paint_str(done, color::Color::Green); }
    }

    println!(" Enter new message for task:");
    print!(" {} {} {}\n {}", id, done, task.text, point);
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut new_message).expect("Failed to read new message");

    task.text = new_message.trim().to_string();

    println!("\nNew task:");
    println!(" {} {} {}", id, done, task.text);

    storage::save_storage(&s);
}

pub fn print_tasks() {
    if !general::is_initialized() {
        general::init_first();
        return;
    }

    let s = storage::load_storage();
    if s.tasks.is_empty() {
        general::nothing_was_found();
        return
    }
    let mut tasks: Vec<_> = s.tasks.iter().collect();
    tasks.sort_by(|a, b| a.0.cmp(&b.0));

    for task in tasks {
        let id = color::paint_str(task.0.to_string(), color::Color::Yellow);
        let message = task.1.text.to_string();
        let mut done: String;
        match task.1.done {
            false => { done = "[ ]".to_string(); done = color::paint_str(done, color::Color::Red); }
            true => { done = "[x]".to_string(); done = color::paint_str(done, color::Color::Green); }
        }
        println!(" {} {} {}", id, done, message);
    }
    println!();
}