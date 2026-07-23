use std::fs::write;

const TODO_HELP: &str =
    "Basic todo app\
    A quick CLI todo app—and I'm not sure what else.\
    Available commands:\
        add: Add a new task.\
        edit: Edit an existing task\
        list: View the full list of available tasks\
        delete: Delete an existing task\
        change status: Change the status of an existing task";

enum Status{
    Pending,
    Completed,
}

struct Task{
    title: String,
    description: String,
    status: Status
}

impl Task{
    fn new(title: String, description: String) -> Self{
        Self{
            title,
            description,
            status: Status::Pending,
        }
    }
}

struct Todo{
    todo: Vec<Task>
}

impl Todo{
    fn new() -> Self{
        Self{
            todo: Vec::new(),
        }
    }
    fn new_to_do(&mut self, to_do: Task){
        self.todo.push(to_do);
    }
}