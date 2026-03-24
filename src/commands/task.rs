use std::{collections::HashMap, io::{self, Write}};

use crate::{storage, color, commands::general};

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
    let color_message = color::paint_str(t.text.clone(), color::Color::Cyan);
    let color_done = color::paint_str("[ ]".to_string(), color::Color::Red);
    let task_cmd = color::paint_str("chant task".to_string(), color::Color::Yellow);
    println!("New taks added:");
    println!(" {} - {} {}\n", color_id, color_done, color_message);
    println!("To see all of the tasks you can run {task_cmd}");

    s.tasks.insert(id.to_string(), t);

    storage::save_storage(&s);
}

pub fn remove_task(id: String) {
    if !general::is_initialized() {
        general::init_first();
        return;
    }
    let mut s = storage::load_storage();
    if !s.tasks.contains_key(&id) {
        println!("task with this id doesn't exists");
        return;
    }
    s.tasks.remove(&id);
    storage::save_storage(&s);
}

pub fn remove_all() {
    if !general::is_initialized() {
        general::init_first();
        return;
    }
    let mut s = storage::load_storage();
    s.tasks = HashMap::new();

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
    print!(" {} - {} {}\n {}", id, done, task.text, point);
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut new_message).expect("Failed to read new message");

    task.text = new_message.trim().to_string();

    println!("\nNew task:");
    println!(" {} - {} {}", id, done, task.text);

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
    for task in s.tasks {
        let id = color::paint_str(task.0, color::Color::Yellow);
        let message = task.1.text;
        let mut done: String;
        match task.1.done {
            false => { done = "[ ]".to_string(); done = color::paint_str(done, color::Color::Red); }
            true => { done = "[x]".to_string(); done = color::paint_str(done, color::Color::Green); }
        }
        println!(" {} - {} {}", id, done, message);
    }
    println!();
}