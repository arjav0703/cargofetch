mod checks;
pub mod parse;

fn main() {
    checks::env_check();
    parse::init();
}
