pub const TODO_HELP: &str =
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

pub struct Task{
    title: String,
    description: String,
    status: Status
}

impl Task{
    pub fn new(title: String, description: String) -> Self{
        Self{
            title,
            description,
            status: Status::Pending,
        }
    }
}

pub struct Todo{
    todo: Vec<Task>
}

impl Todo{
    pub fn new() -> Self{
        Self{
            todo: Vec::new(),
        }
    }

    pub fn new_task(&mut self, task: Task){
        self.todo.push(task);
    }

    pub fn list(&self){
        for task in &self.todo {
            println!("title: {}", task.title);
            println!("Description: {}", task.description);

            match task.status {
                Status::Pending => println!("Status: PENDING"),
                Status::Completed => println!("Status: COMPLETED"),
            }
            println!("---------------------------------------------------")
        }
    }

    pub fn changes_status(&mut self, id: u8){
        let temp = &mut self.todo[id as usize];

        match temp.status {
            Status::Pending => temp.status = Status::Completed,
            Status::Completed => temp.status = Status::Pending,
        }
    }

    pub fn delete_task(&mut self, id: u8){
        self.todo.remove(id as usize);
    }

}


