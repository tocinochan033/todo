mod todo;

fn main() {
    let mut a = todo::Todo::new();


    //adding a lot of task
    let t1 = todo::Task::new("title 1".to_string(),"description 1".to_string());
    todo::Todo::new_task(&mut a, t1);

    let t2 = todo::Task::new("title 2".to_string(),"description 2".to_string());
    todo::Todo::new_task(&mut a, t2);

    let t3 = todo::Task::new("title 3".to_string(),"description 3".to_string());
    todo::Todo::new_task(&mut a, t3);

    let t4 = todo::Task::new("title 4".to_string(),"description 4".to_string());
    todo::Todo::new_task(&mut a, t4);


    todo::Todo::consult(&a);
}
