use std::{fs, io::{self, stdin}, path::Path};

#[derive(Debug)]
struct Task {
    name: String,
    completed: bool,
}

fn main() {
    let mut task_collection = Vec::<Task>::new();
        loop {
            match take_generic_command().as_str() {
                "N" | "n" => {
                    match create_task() {
                        Ok(task) => task_collection.push(task),
                        Err(e) => {
                            eprintln!("Error creating task. {}", e);
                            continue;
                        }
                    };
                }
                "V" | "v" => {
                    if task_collection.is_empty() {
                        println!("There are no tasks right now.");
                        continue;
                    } else {
                        print_tasks(&task_collection);
                    }
                }
                "Q" | "q" => {
                    std::process::exit(0);
                }
                _ => {
                    println!("Invalid Command, please enter one of the valid commands.");
                    continue;
                }
            };
        }
}

fn take_generic_command() -> String {
    let mut command_buf = String::new();

    println!("Valid Commands:");
    println!("N/n -> New Task");
    println!("V/v -> View Tasks");

    stdin().read_line(&mut command_buf).expect("Failed to read command.");

    command_buf.trim().to_string()   
}

fn create_task() -> Result<Task, io::Error> {
    let mut task_name_buf = String::new();
    println!("Enter Task Name: ");
    stdin().read_line(&mut task_name_buf)?;

    let new_task = Task {
        name: task_name_buf.trim().to_string(),
        completed: false 
    };

    Ok(new_task)
}

fn print_tasks(task_col: &Vec<Task>) {
    let task_iter = task_col.iter();

    for task in task_iter {
        println!("{:#?}", task);
    }   
}

fn write_to_file() {
    todo!()
}