pub mod commands;

pub fn run() {
    let arg = Some("my_project"); // Example argument, replace with actual CLI parsing
    commands::init::handle(arg);
}