
enum Status{
    Pending,
    Completed,
}

struct Write{
    title: String,
    description: String,
    status: Status
}

struct Todo{
    todo: Vec<Write>
}