use std::env;
use std::fs;

fn main() 
{
    let args:Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut numbers:Vec<u32> = Vec::new();

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.lines();
    for line in lines
    {
        let number:u32 = line.parse().expect("Not a number");

        numbers.push(number);
    }

    puzzle1(&numbers);
    puzzle2(&numbers);
}

fn puzzle1(numbers:&Vec<u32>)
{
    for i in 0..numbers.len() - 1
    {
        let first = &numbers[i];
        for j in i + 1 .. numbers.len()
        {
            let second = &numbers[j];
            if first + second == 2020
            {
                println!("Result puzzle 1: {}", first * second);
            }
        }
    }
}

fn puzzle2(numbers:&Vec<u32>)
{
    for i in 0..numbers.len() - 2
    {
        let first = &numbers[i];
        for j in i + 1..numbers.len() - 1
        {
            if i == j { continue; }

            let second = &numbers[j];
            for k in j + 1..numbers.len()
            {
                if j == k { continue; }

                let third = &numbers[k];

                if first + second + third == 2020
                {
                    println!("Result puzzle 2: {}", first * second * third);
                }
            }
        }
    }
}
