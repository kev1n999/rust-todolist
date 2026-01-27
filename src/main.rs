use std::io; 
use serde::{Deserialize, Serialize}; 
use std::fs::File;
use std::io::{Write};

// TodoList in rust

#[derive(Debug, Serialize, Deserialize)]
enum Status {
  Pending,
  Completed,
}

#[derive(Debug, Serialize, Deserialize)]
enum Priority {
  Low,
  Medium,
  High,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
  name: String,
  description: String,
  status: Status,
  priority: Priority, 
}

#[derive(Debug, Serialize, Deserialize)]
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

  fn save_tasks(&self) -> Result<(), Box<dyn std::error::Error>> {
    let mut file_tasks = File::create("tasks.json")?;
    
    let json = serde_json::to_string(&self.tasks)?;
    file_tasks.write_all(json.as_bytes())?;

    Ok(())
  }
}

fn main() {
  let mut todo = TodoList {
    tasks: Vec::new(),
    max: 10, 
  }; 

  let task = Task::new("louça".to_string(), "sla".to_string(), Status::Pending, Priority::Low);
  let task2 = Task::new("louasdada".to_string(), "slasdada".to_string(), Status::Completed, Priority::Low);
  
  todo.add_new_task(task);
  todo.add_new_task(task2);
  
  let tasks = todo.get_all_tasks();
  
  match tasks {
    Some(task) => println!("{:?}", task),
    None => println!("An error ocurred"), 
  }

  todo.save_tasks();
}