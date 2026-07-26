use std::io;
use crate::todo;
use crate::todo::Todo;

pub fn add_task(todo: &mut Todo){
    let mut title = String::new();
    let mut description = String::new();

    eprint!("Title: ");
    io::stdin()
        .read_line(&mut title)
        .expect("error entry title");
    eprint!("Description: ");
    io::stdin()
        .read_line(&mut description)
        .expect("error entry description");

    let task_obj = todo::Task::new(title, description);
    Todo::new_task(todo, task_obj);
}


pub fn delete_task(todo: &mut Todo) -> u8{
    Todo::list(todo); //show all task available

    eprint!("SELECT TASK: ");
    let mut entry_id = String::new();
    io::stdin()
        .read_line(&mut entry_id)
        .expect("error entry id");

    entry_id.trim().parse::<u8>().unwrap()
}

pub fn edit_task(todo: &mut Todo){
    'label: loop {
        Todo::list(todo); //show all task available
        eprint!("SELECT TASK: ");

        let mut entry_id = String::new();
        io::stdin()
            .read_line(&mut entry_id)
            .expect("error entry id");

        eprint!("En cual aspecto deseas modificar?");
        let mut task_entry = String::new();
        io::stdin()
            .read_line(&mut task_entry)
            .expect("error entry modification");

        if task_entry == "title\r\n" || task_entry == "description\r\n" {
            eprint!("Modification: ");
            let mut modification_entry = String::new();
            io::stdin()
                .read_line(&mut modification_entry)
                .expect("error entry modification");

            Todo::edit(todo, entry_id.trim().parse::<u8>().unwrap(), task_entry, modification_entry);
            break 'label;
        }else { println!("Favor de seleccionar una opcion valida"); std::thread::sleep(std::time::Duration::from_millis(3000));
        }
    }

}


pub fn changes_status_to_do(todo: &mut Todo){
    Todo::list(todo);
    eprint!("SELECT TASK TO CHANGES THE STATUS: ");

    let mut entry_id = String::new();
    io::stdin()
        .read_line(&mut entry_id)
        .expect("error entry id");

    Todo::changes_status(todo, entry_id.trim().parse::<u8>().unwrap());
}