use aoc_2015::cli::{self, Arguments};

fn main() {
    let args: Arguments = cli::get_args();
    aoc_2015::run(&args);
}
