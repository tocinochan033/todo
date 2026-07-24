use std::io;
use crate::todo;
use crate::todo::Todo;

pub fn add_task(todo: &mut Todo){
    let mut title = String::new();
    let mut description = String::new();

    io::stdin()
        .read_line(&mut title)
        .expect("error entry title");
    io::stdin()
        .read_line(&mut description)
        .expect("error entry description");

    let task_obj = todo::Task::new(title, description);
    todo::Todo::new_task(todo, task_obj);
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