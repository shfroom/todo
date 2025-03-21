use colored::*;
use std::io::Write;

fn main() {
    let mut todos = vec![];
    loop {
        let mut line = String::new();
        println!("{} Show todos\n{} Add new\n{} Delete todo\n{} Exit\n{} Enter the action to be done: ", "[1]".bold(), "[2]".bold(), "[3]".bold(), "[4]".bold(), "?".blue());
        std::io::stdin().read_line(&mut line).expect("Failed to read line");
        match line.trim().parse::<String>() {
            Ok(choice) => {
                if choice == "1" {
                    for (e, todo) in todos.iter().enumerate() {
                        println!("[{}]: {}", (e+1).to_string().blue(), todo);
                    }
                } else if choice == "2" {
                    let todo = get_todo();
                    println!("{} Todo added: {}", "+".green(), todo);
                    todos.push(todo);
                    save_todos(todos.clone());
                } else if choice == "3" {
                    let mut line = String::new();
                    println!("{} Enter the todo to be deleted: ", "?".blue());
                    std::io::stdin().read_line(&mut line).expect("Failed to read line");
                    match line.trim().parse::<usize>() {
                        Ok(todo) => {
                            todos.remove(todo-1);
                        }
                        Err(_) => { eprintln!("{}", "Invalid number — please try again.".red()); }
                    }
                    save_todos(todos.clone());
                } else if choice == "4" {
                    break;
                } else {
                    eprintln!("{}", "Invalid choice — please try again.".red()); 
                }
            },
            Err(_) => { eprintln!("{}", "Invalid choice — please try again.".red()); }
        }
    }
}

fn get_todo() -> String {
    let mut line = String::new();
    println!("{} Enter the todo to be added: ", "?".blue());
    std::io::stdin().read_line(&mut line).expect("Failed to read line");
    return line
}

fn save_todos(todos: Vec<String>) {
    let mut file = std::fs::File::create("todos.txt").expect("Failed to create file");
    for todo in todos {
        file.write_all(todo.as_bytes()).expect("Failed to write to file");
    }
}