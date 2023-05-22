pub mod cli;
pub mod projects;

fn main() {
    projects::init();
    cli::run();
}
