use std::io; 
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Write};
use std::process::Command;

// TodoList in rust

// Enum to status options
#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Status {
  Pending,
  Completed,
}

// Enum to priority options
#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Priority {
  Low,
  Medium,
  High,
}

// Enum to filter options
enum Filter {
  Name(String),
  Status(Status),
  Priority(Priority),
  Id(String),
}

// Enum for todolist commands
enum TaskCommands {
  // Create a new task
  CreateTask,
  // Edit a specific task 
  EditTask, 
  // Delete a specific task 
  DeleteTask,
  // Find all tasks,
  FindTasks,
  // Find a specific task 
  FindTask,
  // Delete all tasks 
  DeleteAllTask,
  // Save tasks in json file 
  SaveTasks, 
  // Quit the program
  Quit, 
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
  name: String,
  description: String,
  status: Status,
  priority: Priority, 
  id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
  tasks: Vec<Task>,
  max: usize,
}

impl Task {
  // Function to create a new task
  fn new(name: String, description: String, status: Status, priority: Priority, id: String) -> Self {
    Task { name, description, status: Status::Pending, priority, id }
  }
}

impl TodoList {
  // Function to add a new task in the list
  fn add_new_task(&mut self, task: Task) -> Result<(), String> {
    if self.tasks.len() > self.max {
      return Err("".to_string());
    } 

    self.tasks.push(task);
    Ok(())
  }

  // Function to get all tasks in the list
  fn get_all_tasks(&self) -> Option<&[Task]> {
    if self.tasks.is_empty() {
      return None; 
    }

    Some(&self.tasks) 
  }

  // Function to save task in the json file
  fn save_tasks(&self) -> Result<(), Box<dyn std::error::Error>> {
    let mut file_tasks = File::create("tasks.json")?;
    
    let json = serde_json::to_string(&self.tasks)?;
    file_tasks.write_all(json.as_bytes())?;

    Ok(())
  }

  // Function to get a task by filter(Name, Status, Priority)
  fn get_task_by_filter(&self, filter: Filter) -> Option<Vec<&Task>> {
    if self.tasks.is_empty() {
      return None; 
    }

    let tasks: Vec<&Task> = match filter {
      Filter::Name(name) => self.tasks.iter().filter(
        |task| task.name.as_str() == name
      ).collect(),

      Filter::Status(status) => self.tasks.iter().filter(
        |task| task.status == status
      ).collect(),

      Filter::Priority(priority) => self.tasks.iter().filter(
        |task| task.priority == priority 
      ).collect(),

      Filter::Id(id) => self.tasks.iter().filter(
        |task| task.id == id
      ).collect(),

      _ => return None,
    };

    Some(tasks)
  }

  fn delete_task(&mut self) -> Result<(), String> {
    if self.tasks.is_empty() {
      return Err("The todolist is empty".to_string());
    }

    let mut id = String::new();

    println!("Type the stack id to delete: ");
    io::stdin()
      .read_line(&mut id);

    let tasksFounded = self.get_task_by_filter(Filter::Id(id.clone())); 

    match tasksFounded {
      Some(tasks) => {
        println!("Tasks founded:\n{:?}", tasks);

        let mut answer = String::new();

        io::stdin()
          .read_line(&mut answer);

        match answer.trim().to_lowercase().as_str() {
          "yes" => {
            self.tasks.retain(|task| task.id != id.to_string());
            return Ok(());
          },
          "no" => return Ok(()),

          _ => println!("Invalid answer"),
        }
      }, 
      _ => println!("Tasks not found by this id"),
    }

    Ok(())
  }
}

fn sys_command(command: &str) {
  let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .arg("/C")
      .arg(command)
      .output()
      .expect("An error ocurred to execute this command");

  } else {
    Command::new("sh")
      .arg("-c")
      .arg(command)
      .output()
      .expect("An error ocurred to execute this command");
  };
}

fn get_priority(priority: &str) -> Option<Priority> {
  match priority.trim().to_lowercase().as_str() {
    "low" => Some(Priority::Low),
    "medium" => Some(Priority::Medium),
    "high" => Some(Priority::High), 

    _ => None, 
  }
}

// Get atributtes to create a new task
fn get_atributtes(
  name: &mut String,
  description: &mut String,
  priority: &mut String,
  id: &mut String,
) {
  println!("[Task name] Type the task name: ");
  io::stdin()
    .read_line(name);

  println!("[Task description] Type the task description: ");
  io::stdin()
    .read_line(description);
  
  println!("[Task priority][Low, Medium, High] Type the task priority: ");
  io::stdin()
    .read_line(priority);

  println!("[Task id] Type the task id: ");
  io::stdin()
    .read_line(id);
}

fn display_commands() {
  println!("
    [0] Create a new task
    [1] Edit a task 
    [2] Delete a specific task 
    [3] Find a specific task 
    [4] Delete all tasks 
    [5] Save tasks in json
    [5] quit 
  ");
}

fn create_task(todo: &mut TodoList, name: &mut String, description: &mut String, priority: &mut String, id: &mut String) {
  get_atributtes(name, description, priority, id);

  if let Some(priority) = get_priority(&priority) {
    let task = Task::new(name.to_string(), description.to_string(), Status::Pending, priority, id.to_string());

    match todo.add_new_task(task) {
      Ok(()) => println!("Task created!"),
      _ => println!("Task doesn't created!"),
    }
  }
}

fn main() {
  let mut name = String::new();
  let mut description = String::new();
  let mut priority = String::new();
  let mut id = String::new();

  let mut command = String::new(); 
  let mut todo = TodoList {
    tasks: Vec::new(),
    max: 10,
  };

  loop {
    display_commands();
    command.clear();

    io::stdin()
      .read_line(&mut command); 

    match command.trim().chars().next() {
      Some('0') => create_task(&mut todo, &mut name, &mut description, &mut priority, &mut id),
      Some('2') => todo.delete_task().expect("An error ocurred to delete the tasks"),
      Some('5') => panic!("Quited."), 
      _ => println!("Invalid command!"), 
    }
  }
}
