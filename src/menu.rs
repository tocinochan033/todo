use std::io;
use crate::todo;
use crate::todo::Todo;

pub fn add_task(todo: &mut Todo){
    let mut title = String::new();
    let mut description = String::new();

    println!("name Title: ");
    io::stdin()
        .read_line(&mut title)
        .expect("error entry title");
    println!("Description: ");
    io::stdin()
        .read_line(&mut description)
        .expect("error entry description");

    let task_obj = todo::Task::new(title, description);
    Todo::new_task(todo, task_obj);
}


pub fn delete_task(todo: &mut Todo) -> u8{
    println!("SELECT TASK: ");
    Todo::list(todo);

    println!(": ");
    let mut entry_id = String::new();
    io::stdin()
        .read_line(&mut entry_id)
        .expect("error entry id");

    entry_id.parse::<u8>().unwrap()
}

pub fn edit_task(todo: &mut Todo){
    println!("SELECT TASK: ");
    Todo::list(todo);

    println!(": ");
    let mut entry_id = String::new();
    io::stdin()
        .read_line(&mut entry_id)
        .expect("error entry id");

    println!("En cual aspecto deseas modificar?");
    println!(": ");
    let mut task_entry = String::new();
    io::stdin()
        .read_line(&mut task_entry)
        .expect("error entry modification");

    println!("Mod");
    println!(": ");
    let mut modification_entry = String::new();
    io::stdin()
        .read_line(&mut modification_entry)
        .expect("error entry modification");
    
    todo::Todo::edit(todo, entry_id.parse::<u8>().unwrap(), task_entry, modification_entry);
}


pub fn changes_status_to_do(todo: &mut Todo){
    println!("SELECT TASK TO CHANGES THE STATUS: ");
    Todo::list(todo);

    println!(": ");
    let mut entry_id = String::new();
    io::stdin()
        .read_line(&mut entry_id)
        .expect("error entry id");

    Todo::changes_status(todo, entry_id.parse::<u8>().unwrap());
}