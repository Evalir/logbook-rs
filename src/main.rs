use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use structopt::StructOpt;

const LOGBOOK_PATH: &str = ".logbook.json";

#[derive(Debug, StructOpt)]
enum SubCommands {
    AddProject,
    Log(LogOpts),
    DeleteLog(DeleteLogOpts),
    DeleteProject(DeleteProjectOpts),
}

#[derive(Debug, StructOpt)]
struct LogOpts {
    #[structopt(short, long, help = "Project name")]
    project: String,
}

#[derive(Debug, StructOpt)]
struct DeleteLogOpts {
    #[structopt(short, long, help = "Project name")]
    name: String,
    #[structopt(short, long, help = "log id")]
    id: String,
}

#[derive(Debug, StructOpt)]
struct DeleteProjectOpts {
    #[structopt(short, long, help = "Project name")]
    name: String,
}

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
struct Logbook {
    created_at: String,
    projects: HashMap<String, Project>,
}

fn load_or_create_logbook() -> Logbook {
    let logbook_path = get_logbook_dir();
    let logbook = match fs::read_to_string(get_logbook_dir()) {
        Err(_) => {
            println!("Could not open logbook, creating one.");
            let mut file = fs::File::options()
                .read(true)
                .write(true)
                .create(true)
                .open(logbook_path.clone())
                .expect("Could not create logbook file");
            let initial_state = Logbook {
                created_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Could not get current timestamp")
                    .as_millis()
                    .to_string(),
                projects: HashMap::new(),
            };
            file.write_all(
                serde_json::to_string(&initial_state)
                    .expect("Could not initialize logbook")
                    .as_bytes(),
            )
            .expect("Could not init logbook");
            let mut buf = String::new();
            file.read_to_string(&mut buf)
                .expect("Could not read logbook from file");
            serde_json::from_str(&buf).expect("Malformed logbook on init. This is a bug")
        }
        Ok(unserialized_logbook) => {
            serde_json::from_str(&unserialized_logbook).expect("Malformed logbook")
        }
    };
    logbook
}

fn write_to_logbook(contents: &str) -> Result<(), std::io::Error> {
    let mut file = fs::File::create(get_logbook_dir()).expect("Could not open logbook file");
    file.write_all(&contents.as_bytes())
}

fn get_logbook_dir() -> String {
    let home_dir = dirs::home_dir().unwrap();
    let home_dir = home_dir.to_str().unwrap();
    format!("{}/{}", home_dir, LOGBOOK_PATH)
}

fn add_project(name: &str) {
    let mut logbook = load_or_create_logbook();
    logbook.projects.insert(
        name.to_string(),
        Project {
            name: name.to_string(),
            logs: Vec::new(),
        },
    );

    write_to_logbook(&serde_json::to_string(&logbook).expect("Could not serialize logbook"))
        .expect("Could not write contents")
}

fn add_log(project: &str, text: &str) {
    let mut logbook = load_or_create_logbook();
    let project = logbook
        .projects
        .get_mut(project)
        .expect("Nonexistent project");
    project.logs.push(Log {
        id: project.logs.len().into(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string(),
        text: text.to_string(),
    });
    write_to_logbook(&serde_json::to_string(&logbook).expect("Could not serialize logbook"))
        .expect("Could not write contents");
    println!("new logbook: {:#?}", logbook);
}

fn delete_log(project: &str, id: i32) {}

fn delete_project(name: &str) {}

fn main() {
    println!("{:#?}", load_or_create_logbook());
}
