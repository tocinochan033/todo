mod todo;
mod menu;

fn main() {
    let mut a = todo::Todo::new();

    let mut entry = String::new();
    std::io::stdin()
        .read_line(&mut entry).
        expect("Error stdin");

    match entry.as_str() {
        "add" => menu::add_task(),
        "list" => todo::Todo::list(&a),
        "delete" => {
            let id: u8 = menu::delete_task(&mut a);
            todo::Todo::delete_task(&mut a, id);
        },
        _ => println!("Seleccione una opcion valida\n {}", todo::TODO_HELP),
    }
}
