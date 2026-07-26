# Simple TodoApp
A CLI application to manage your task list.

## Installation and use
```bash
cargo run
```

## Commands
- **add**: Add a new task. Ask for title and description.
- **list**: Shows all tasks with their status (PENDING or COMPLETED).
- **edit**: Edit the title or description of an existing task.
- **delete**: Delete a task.
- **changes status**: Change the status of a task between pending and completed.

## How it works
The program maintains a list of tasks in memory. Each task has:
- A title
- A description
- A status (Pending or Completed)

Use menu commands to create, view, modify or delete tasks. Data is lost when closing the application.
