pub fn solve(contents:&String) -> (u32, u64)
{
    let mut adapters:Vec<u32> = Vec::new();
    adapters.push(0);

    for line in contents.lines()
    {
        let adapter:u32 = line.parse().unwrap();
        adapters.push(adapter);
    }

    adapters.sort();

    adapters.push(adapters[adapters.len() - 1] + 3);

    return (puzzle1(&adapters), puzzle2(&adapters));
}

fn puzzle1(adapters:&Vec<u32>) -> u32
{
    let mut ones = 0;
    let mut threes = 0;
    let mut previous = 0;

    for adapter in adapters
    {
        let diff = adapter - previous;
        if diff == 1
        {
            ones += 1;
        }
        else if diff == 3
        {
            threes += 1;
        }

        previous = *adapter;
    }

    return ones * threes;
}

fn puzzle2(adapters:&Vec<u32>) -> u64
{
    let mut series = 0;
    let mut possibilities = 1;
    let mut previous = 0;

    for adapter in adapters
    {
        let diff = adapter - previous;
        if diff == 1
        {
            series += 1;
        }
        else if diff == 3
        {
            if series > 0
            {
                let mut poss = 2u64.pow(series - 1);
                if series == 4
                {
                    poss -= 1;
                }

                possibilities *= poss;
                series = 0;
            }
        }
        previous = *adapter;
    }

    return possibilities;
}
