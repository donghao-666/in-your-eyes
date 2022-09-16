use crate::app::run_app;

pub mod app;
pub mod brightness_services;
pub mod gamma_color;
pub mod gamma_control;
pub mod monitor;
pub mod ns_number;
pub mod utils;

fn main() {
    run_app()
}
