pub fn solve(contents:&String) -> (u64, u64)
{
    let mut numbers:Vec<u64> = Vec::new();

    for line in contents.lines()
    {
        let number:u64 = line.parse().unwrap();
        numbers.push(number);
    }

    let invalid = puzzle1(&numbers);
    return (invalid, puzzle2(&numbers, invalid));
}

fn puzzle1(numbers:&Vec<u64>) -> u64
{
    let preamble_length = 25;

    'outer: for n in preamble_length..numbers.len()
    {
        for i in n - preamble_length..n - 1
        {
            for j in i + 1..n
            {
                if numbers[i] + numbers[j] == numbers[n]
                {
                    continue 'outer;
                }
            }
        }

        return numbers[n] as u64;
    }

    return 0;
}

fn puzzle2(numbers:&Vec<u64>, invalid:u64) -> u64
{
    let mut n;
    let mut l;
    let mut h;
    for i in 0..numbers.len()
    {
        n = numbers[i];
        l = n;
        h = n;
        let mut j = i + 1;
        while n < invalid
        {
            n += numbers[j];

            if numbers[j] < l
            {
                l = numbers[j];
            }
            if numbers[j] > h
            {
                h = numbers[j];
            }

            j += 1;
        }

        if n == invalid
        {
            return l + h;
        }
    }

    return 0;
}
