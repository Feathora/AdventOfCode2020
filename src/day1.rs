pub fn solve(contents:&String)
{
    let mut numbers:Vec<u32> = Vec::new();
    
    let lines = contents.lines();
    for line in lines
    {
        let number:u32 = line.parse().expect("Not a number");

        numbers.push(number);
    }

    let result1 = puzzle1(&numbers);
    println!("Result puzzle 1: {}", result1);

    let result2 = puzzle2(&numbers);
    println!("Result puzzle 2: {}", result2);
}

fn puzzle1(numbers:&Vec<u32>) -> u32
{
    for i in 0..numbers.len() - 1
    {
        let first = &numbers[i];
        for j in i + 1 .. numbers.len()
        {
            let second = &numbers[j];
            if first + second == 2020
            {
                return first * second;
            }
        }
    }

    return 0;
}

fn puzzle2(numbers:&Vec<u32>) -> u32
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
                    return first * second * third;
                }
            }
        }
    }

    return 0;
}
