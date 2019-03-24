mod args;
use crate::args::AppArgs;
use structopt::StructOpt;
use std::fs;

fn main() {
    let args = AppArgs::from_args();
    println!("{:#?}", args);

    for i in 0..args.files {
        let path = args.path.join(format!("{}.txt", i));
        let file = fs::read_to_string(&path)
            .expect(&format!("File does not exist: '{:?}'", path));
        let mut numbers = file.lines()
            .filter_map(|line| line.parse::<i32>().ok())
            .collect::<Vec<_>>();

        if args.sort {
            numbers.sort()
        }
    }

    println!("Program ended successfully")
}
