use std::convert::TryInto;

struct Password
{
    pass: String,
    req: char,
    min: u32,
    max: u32
}

pub fn solve(contents:&String)
{
    let mut passwords:Vec<Password> = Vec::new();

    for line in contents.lines()
    {
        let parts:Vec<&str> = line.split_whitespace().collect();
        let rule = parts[0];
        let req = parts[1].chars().nth(0).expect("Error getting nth character");
        
        let minmax:Vec<&str> = rule.split('-').collect();
        let min:u32 = minmax[0].parse().expect("Not a number");
        let max:u32 = minmax[1].parse().expect("Not a number");

        let pass = parts[2];

        let p = Password 
        {
            pass: pass.to_string(),
            req: req,
            min: min,
            max: max
        };

        passwords.push(p);
    }

    let result1 = puzzle1(&passwords);
    println!("Result 1: {}", result1);

    let result2 = puzzle2(&passwords);
    println!("Result 2: {}", result2);
}

fn puzzle1(passwords:&Vec<Password>) -> u32
{
    let mut result = 0;
    for password in passwords
    {
        let count = password.pass.matches(password.req).count();
        if count >= password.min.try_into().unwrap() && count <= password.max.try_into().unwrap()
        {
            result += 1;
        }
    }

    return result;
}

fn puzzle2(passwords:&Vec<Password>) -> u32
{
    let mut result = 0;
    for password in passwords
    {
        let first:usize = (password.min - 1).try_into().unwrap();
        let second:usize = (password.max - 1).try_into().unwrap();
        if (password.pass.chars().nth(first).unwrap() == password.req) ^ (password.pass.chars().nth(second).unwrap() == password.req)
        {
            result += 1;
        }
    }

    return result;
}
