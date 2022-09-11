use std::{fs, io::Read};

use structopt::StructOpt;

const LOGBOOK_PATH: &str = "~/.logbook.json";

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

fn add_project(name: &str) {}

fn add_log(project: &str, text: &str) {}

fn delete_log(project: &str, id: i32) {}

fn delete_project(name: &str) {}

fn load_or_create_logbook() -> String {
    let mut buffer = String::new();
    fs::File::open(LOGBOOK_PATH)
        .unwrap_or_else(|_| fs::File::create(LOGBOOK_PATH).unwrap())
        .read_to_string(&mut buffer)
        .expect("Could not read logbook");
    buffer
}

fn main() {
    println!("Hello, world!");
}
