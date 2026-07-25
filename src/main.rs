mod todo;
mod menu;

fn main() {
    let mut a = todo::Todo::new();

    loop {
        println!("Todo list in rust :v");
        print!("entry: ");

        let mut entry = String::new();
        std::io::stdin()
            .read_line(&mut entry).
            expect("Error stdin");

        match entry.trim() {
            "add" => menu::add_task(&mut a),
            "list" => todo::Todo::list(&a),
            "delete" => {
                let id: u8 = menu::delete_task(&mut a);
                todo::Todo::delete_task(&mut a, id);
            },
            "edit" => menu::edit_task(&mut a),
            "changes status" => menu::changes_status_to_do(&mut a),
            _ => {
                clear_screen();
                println!("SELECCIONE UNA OPCION VALIDA.\n {}", todo::TODO_HELP);
                std::thread::sleep(std::time::Duration::from_millis(3000));
            },
        }
    }

}

pub fn clear_screen() {
    // \x1B[2J borra toda la pantalla
    // \x1B[0;0f mueve el cursor a la esquina superior izquierda
    print!("\x1B[2J\x1B[0;0f");
}
