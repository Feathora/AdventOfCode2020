use std::env;
use std::fs;

mod day11;
use day11::solve;

fn main() 
{
    let args:Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let results = solve(&contents);
    println!("Result 1: {}\nResult 2: {}", results.0, results.1);
}
