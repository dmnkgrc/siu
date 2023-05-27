pub mod cli;
pub mod projects;
pub mod tools;

fn main() {
    projects::init();
    cli::run();
}
