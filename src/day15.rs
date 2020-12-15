use std::collections::HashMap;

pub fn solve(contents:&String) -> (u32, u32)
{
    let mut numbers:Vec<u32> = Vec::new();

    for number_str in contents.split(',')
    {
        let number:u32 = number_str.parse().unwrap();
        numbers.push(number);
    }

    return (puzzle1(&numbers), puzzle2(&numbers));
}

fn puzzle1(numbers:&Vec<u32>) -> u32
{
    return play_game(&numbers, 2020);
}

fn puzzle2(numbers:&Vec<u32>) -> u32
{
    return play_game(&numbers, 30000000);
}

fn play_game(numbers:&Vec<u32>, rounds:usize) -> u32
{
    let mut history:HashMap<u32, u32> = HashMap::new();
    let mut last_inserted = 0;
    let mut last_age = 0;

    for i in 0..rounds
    {
        if i < numbers.len()
        {
            history.insert(numbers[i], i as u32);
            last_inserted = numbers[i];
            last_age = 0;
        }
        else
        {
            let next = last_age;
            if history.contains_key(&next)
            {
                last_age = i as u32 - history.get(&next).unwrap();
            }
            else
            {
                last_age = 0;
            }

            history.insert(next, i as u32);
            last_inserted = next;
        }
    }

    return last_inserted;
}
