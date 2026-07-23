use std::collections::hash_map::Entry;

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

struct Write{
    title: String,
    description: String,
    status: Status
}

impl Write{
    fn addToDo(&mut self, title_entry: String, description_entry: String) {
        self.title = title_entry;
        self.description = description_entry;
        self.status = Status::Pending;
    }
}

struct Todo{
    todo: Vec<Write>
}