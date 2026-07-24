use std::io;
use crate::todo;

pub fn add_task(){
    let mut title = String::new();
    let mut description = String::new();

    io::stdin()
        .read_line(&mut title)
        .expect("error entry title");
    io::stdin()
        .read_line(&mut description)
        .expect("error entry description");

    todo::Task::new(title, description);
}
