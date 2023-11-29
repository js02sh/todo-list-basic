use std::io;
use chrono::prelude::*;

struct Task {
    description: String,
    due_date: NaiveDate, //Due date of the task
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("1. Add task");
        println!("2. List tasks");
        println!("3. Exit");

        let mut choice = String::new(); //Read user choice
        io::stdin()
            .read_line(&mut choice)
            .expect("Faild to read line");
        let choice: u32 = choice.trim().parse()
            .expect("Please enter a valid number");

        match choice {
            1 => {
                println!("Enter task description:");
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Faild to read line");

                println!("Enter due date (YYYY-MM-DD): ");
                let mut due_date = String::new();
                io::stdin()
                    .read_line(&mut due_date)
                    .expect("Faild to read line");
                let due_date = NaiveDate::parse_from_str(&due_date.trim(), "%Y-%m-%d")
                    .expect("Please enter a valid date");

                let task = Task {
                    description,
                    due_date,
                };

                tasks.push(task);
                println!("Task added successfully");
            }
            
            2 => {
                println!("\nTasks:");
                for (index,task) in tasks.iter().enumerate() {
                    println!("{}. {} (Due: {})", index + 1, task.description, task.due_date);
                }
                println!();
            }

            3 => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("\nInvalid choice\n");
                continue;
            }
            //println!("Invalid choice"),
        }
    }
}