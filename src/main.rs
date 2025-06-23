pub mod art;
mod checks;
pub mod cli;
pub mod parse;
pub mod size;
pub mod structure;

fn main() {
    checks::env_check();
    parse::init();
}
