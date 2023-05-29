pub mod cli;
pub mod projects;
pub mod shell;
pub mod tools;

fn main() {
    projects::init();
    cli::run();
}
