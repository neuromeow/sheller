mod cli;
mod core;

fn main() {
    if let Err(e) = core::run() {
        println!("{}", e);
        std::process::exit(1);
    }
}
