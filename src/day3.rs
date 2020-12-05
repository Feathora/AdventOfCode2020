pub fn solve(contents:&String)
{
    let width = String::from(contents.lines().nth(0).expect("No first line found")).len();
    let height = contents.lines().count();

    let mut hill = vec![vec![false; height]; width];

    for y in 0..height
    {
        let line = String::from(contents.lines().nth(y).expect("No line found"));
        for x in 0..width
        {
            let c = line.chars().nth(x).expect("No character found");
            hill[x][y] = c == '#';
        }
    }

    let result1 = puzzle1(&hill, height, &(3, 1));
    println!("Result 1: {}", result1);

    let result2 = puzzle2(&hill, height);
    println!("Result 2: {}", result2);
}

fn puzzle1(hill:&Vec<Vec<bool>>, height:usize, slope:&(usize, usize)) -> u32
{
    let mut count = 0;
    let mut x = 0;
    let mut y = 0;

    while y < height
    {
        if hill[x][y]
        {
            count += 1;
        }

        x = (x + slope.0) % hill.len();
        y += slope.1;
    }

    return count;
}

fn puzzle2(hill:&Vec<Vec<bool>>, height:usize) -> u32
{
    let slopes:[(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut result = 1;

    for slope in &slopes
    {
        result *= puzzle1(&hill, height, slope);
    }

    return result;
}
