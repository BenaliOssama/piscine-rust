use std::fs::File;
use std::io::Read;
use json::JsonValue;
use json;
pub use err::*;
mod err;


use std::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}



impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err)=> return Err(Box::new(err::ReadErr{child_err:Box::new(err)})),
        };
        let mut json = String::new();

        file.read_to_string(&mut json).unwrap();

        let parsed = match json::parse(&json) {
            Ok(json) => json,
            Err(err) => return Err(Box::new(err::ParseErr::Malformed(Box::new(err)))),
        };


        let mut tasks = Vec::new();

        let title = parsed["title"].to_string();

        if let JsonValue::Array(arr) = &parsed["tasks"] {
           for item in arr {
                let id = item["id"].as_u32().unwrap_or(0);
                let description = item["description"].as_str().unwrap().to_string();
                let level = item["level"].as_u32().unwrap_or(0);

                tasks.push(Task{id: id, description: description, level: level})
           } 
        }


        if tasks.len() == 0 {
            return Err(Box::new(err::ParseErr::Empty));
        }

        Ok(TodoList{title: title, tasks:tasks})
    }
}

