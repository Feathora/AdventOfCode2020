struct BoardingPass
{
    row_cmds:Vec<bool>,
    col_cmds:Vec<bool>
}

pub fn solve(contents:&String) -> (u32, u32)
{
    let mut passes:Vec<BoardingPass> = Vec::new();

    for line in contents.lines()
    {
        let row = &String::from(line)[..7];
        let col = &String::from(line)[row.len()..];

        let mut pass = BoardingPass
        {
            row_cmds: vec![false; 7],
            col_cmds: vec![false; 3]
        };

        for i in 0..row.len()
        {
            pass.row_cmds[i] = row.chars().nth(i).expect("No character found") == 'B'
        }

        for i in 0..col.len()
        {
            pass.col_cmds[i] = col.chars().nth(i).expect("No character found") == 'R'
        }

        passes.push(pass);
    }

    return (puzzle1(&passes), puzzle2(&passes));
}

fn puzzle1(passes:&Vec<BoardingPass>) -> u32
{
    let mut highest = 0;
    for pass in passes
    {
        let seat_id = solve_binary_space(&pass.row_cmds) * 8 + solve_binary_space(&pass.col_cmds);
        if seat_id > highest
        {
            highest = seat_id;
        }
    }

    return highest;
}

fn puzzle2(passes:&Vec<BoardingPass>) -> u32
{
    let mut seats:Vec<u32> = Vec::new();

    for pass in passes
    {
        let seat_id = solve_binary_space(&pass.row_cmds) * 8 + solve_binary_space(&pass.col_cmds);
        seats.push(seat_id);
    }

    seats.sort();

    for id in seats[0]..seats[seats.len() - 1]
    {
        if !seats.contains(&id) && seats.contains(&(id - 1)) && seats.contains(&(id + 1))
        {
            return id;
        }
    }

    return 0;
}

fn solve_binary_space(input:&Vec<bool>) -> u32
{
    let mut min = 0;
    let mut max = u32::pow(2, input.len() as u32) - 1;

    for b in input
    {
        let range = ((max - min) / 2) + 1;
        if *b
        {
            min += range;
        }
        else
        {
            max -= range;
        }
    }

    return min;
}
