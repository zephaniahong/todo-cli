mod cli;
mod data;
use clap::Parser;
use cli::{Cli, Commands};
use data::TodoItem;
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

fn main() {
    let cli = Cli::parse();

    // create a json db file if it doesn't already exist
    let file_path = "./.db.json";
    let mut options = OpenOptions::new();
    options.write(true).read(true).create(true);
    let mut todos: Vec<TodoItem>;

    match options.open(&file_path) {
        Ok(mut file) => {
            todos = {
                let file_content = fs::read_to_string(file_path).unwrap();
                serde_json::from_str(&file_content).unwrap()
            };
            // println!("File successfully opened/created: {:?}", todos);
            match &cli.command {
                Commands::Create(todo) => {
                    let new_todo = TodoItem::new(todo.item.clone(), todo.completed);
                    todos.push(new_todo);
                    let json_data = serde_json::to_string_pretty(&todos).unwrap();
                    file.write_all(json_data.as_bytes()).unwrap();
                    println!("Successfully created todo: {}", todo.item);
                }
                Commands::List => {
                    println!("Todo Items: ");
                    todos.iter().for_each(|t| println!("{}: {}", t.id, t.item));
                }
            }
        }
        Err(err) => panic!("Error creating/opening db file: {}", err),
    }
}
