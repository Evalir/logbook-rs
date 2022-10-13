use logbook_rs as logbook;
use structopt::StructOpt;

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

fn main() {
    // println!("{:#?}", logbook::load_or_create_logbook());
}
