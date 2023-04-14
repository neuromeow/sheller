mod cli;
mod core;
mod util;
mod names_generator;

fn main() {
    if let Err(e) = core::run() {
        println!("{}", e);
        std::process::exit(1);
    }
}
