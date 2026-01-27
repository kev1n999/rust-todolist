use std::io; 
use serde_json::{Serializer, Deserializer}; 
use std::fmt; 

// TodoList in rust

#[derive(Debug)]
enum Status {
  Pending,
  Completed,
}

#[derive(Debug)]
enum Priority {
  Low,
  Medium,
  High,
}

#[derive(Debug)]
struct Task {
  name: String,
  description: String,
  status: Status,
  priority: Priority, 
}

struct TodoList {
  tasks: Vec<Task>,
  max: usize,
}

impl Task {
  fn new(name: String, description: String, status: Status, priority: Priority) -> Self {
    Task { name, description, status, priority }
  }
}

impl TodoList {
  fn add_new_task(&mut self, task: Task) -> Result<(), String> {
    if self.tasks.len() > self.max {
      return Err("".to_string());
    } 

    self.tasks.push(task);
    Ok(())
  }

  fn get_all_tasks(&self) -> Option<&[Task]> {
    if self.tasks.is_empty() {
      return None; 
    }

    Some(&self.tasks) 
  }
}

fn main() {
  let mut todo = TodoList {
    tasks: Vec::new(),
    max: 10, 
  }; 

  let task = Task::new("louça".to_string(), "sla".to_string(), Status::Pending, Priority::Low);

  let addtask = todo.add_new_task(task);

  match addtask {
    Ok(_) => println!("Task created!"), 
    Err(err) => println!("{:?}", err),
  }

  let tasks = todo.get_all_tasks();
  
  match tasks {
    Some(task) => println!("{:?}", task),
    None => println!("An error ocurred"), 
  }
}