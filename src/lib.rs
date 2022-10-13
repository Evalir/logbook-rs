use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

const LOGBOOK_PATH: &str = ".logbook.json";

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    name: String,
    logs: Vec<Log>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Log {
    id: usize,
    timestamp: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Logbook {
    created_at: String,
    projects: HashMap<String, Project>,
}

impl Logbook {
    pub fn new() -> Logbook {
        Logbook {
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Could not get current timestamp")
                .as_millis()
                .to_string(),
            projects: HashMap::new(),
        }
    }
    pub fn add_project(&mut self, name: &str) {
        self.projects.insert(
            name.to_string(),
            Project {
                name: name.to_string(),
                logs: Vec::new(),
            },
        );
    }

    pub fn add_log(&mut self, project: &str, text: &str) {
        let project = self.projects.get_mut(project).expect("Nonexistent project");
        project.logs.push(Log {
            id: project.logs.len().into(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
                .to_string(),
            text: text.to_string(),
        });
    }

    pub fn delete_log(&mut self, project: &str, id: usize) {
        let project = self.projects.get_mut(project).expect("Nonexistent project");

        if id >= project.logs.len() {
            println!("Invalid ID.");
            return;
        }

        // Remove the log
        project.logs.remove(id);
        // Re-assign IDs as now this vector has shifted,
        // and all IDs beyond the removed one are invalid
        // TODO: Find a better way to assign IDs to avoid this extra step?
        // Might lead into a more complicated data structure conversation,
        // but will avoid doing weird things for now
        for (index, log) in project.logs.iter_mut().enumerate() {
            log.id = index;
        }
    }

    pub fn delete_project(&mut self, name: &str) {
        match self.projects.remove(name) {
            Some(_) => {
                println!("Project {} removed.", name)
            }
            None => {
                println!("Could not find this project.")
            }
        }
    }
}

pub fn get_default_logbook_dir() -> String {
    let home_dir = dirs::home_dir().unwrap();
    let home_dir = home_dir.to_str().unwrap();
    format!("{}/{}", home_dir, LOGBOOK_PATH)
}

fn open_logbook(logbook_path: &str) -> File {
    fs::File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(logbook_path)
        .expect("Could not create logbook file")
}

fn init_logbook(logbook_path: &str) -> Result<(), std::io::Error> {
    let mut file = fs::File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(logbook_path)
        .expect("Could not create logbook file");

    let initial_state = Logbook::new();

    file.write_all(
        serde_json::to_string(&initial_state)
            .expect("Could not initialize logbook")
            .as_bytes(),
    )
    .expect("Could not init logbook");

    Ok(())
}

pub fn load_or_create_logbook(logbook_path: &str) -> Result<Logbook, std::io::Error> {
    let logbook = match fs::read_to_string(logbook_path) {
        Err(_) => {
            println!("Could not find logbook, creating one.");

            match init_logbook(logbook_path) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error creating logbook: {}", e);
                    return Err(e);
                }
            }

            let mut buf = String::new();

            let mut file = open_logbook(logbook_path);

            file.read_to_string(&mut buf)
                .expect("Could not read logbook from file");

            Ok(serde_json::from_str(&buf).expect("Malformed logbook on init. This is a bug"))
        }
        Ok(unserialized_logbook) => {
            Ok(serde_json::from_str(&unserialized_logbook).expect("Malformed logbook"))
        }
    };

    logbook
}

fn write_changes(logbook: &Logbook, logbook_path: &str) -> Result<(), std::io::Error> {
    let mut file = fs::File::create(logbook_path).expect("Could not open logbook file");
    let contents = &serde_json::to_string(&logbook).expect("Could not serialize logbook");
    file.write_all(&contents.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(2 + 2, 4);
    }
}
