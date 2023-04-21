mod cli;
mod core;
mod names_generator;
mod util;

fn main() {
    if let Err(e) = core::run() {
        println!("{}", e);
        std::process::exit(1);
    }
}
