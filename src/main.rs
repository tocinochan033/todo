mod todo;
mod menu;

fn main() {
    let mut a = todo::Todo::new();

    println!("Todo list in rust :v");
    print!("entry: ");

    let mut entry = String::new();
    std::io::stdin()
        .read_line(&mut entry).
        expect("Error stdin");

    match entry.as_str() {
        "add" => menu::add_task(&mut a),
        "list" => todo::Todo::list(&a),
        "delete" => {
            let id: u8 = menu::delete_task(&mut a);
            todo::Todo::delete_task(&mut a, id);
        },
        "changes status" => menu::changes_status_to_do(&mut a),
        _ => println!("SELECCIONE UNA OPCION VALIDA.\n {}", todo::TODO_HELP),
    }
}
