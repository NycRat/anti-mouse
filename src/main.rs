use crate::application::Application;

pub mod application;
pub mod config;
pub mod vec2;

fn main() {
    let application = Application::new();
    application.run();
}
