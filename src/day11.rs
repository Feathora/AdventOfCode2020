pub fn solve(contents:&String) -> (u32, u32)
{
    let width = contents.lines().nth(0).unwrap().len();
    let height = contents.lines().count();

    let mut seats:Vec<Vec<Option<bool>>> = vec![vec![None; height]; width];

    for y in 0..height
    {
        let line = String::from(contents.lines().nth(y).expect("No line found"));
        for x in 0..width
        {
            let c = line.chars().nth(x).expect("No character found");
            if c == 'L'
            {
                seats[x][y] = Some(false);
            }
        }
    }

    return (puzzle1(&seats), puzzle2(&seats));
}

fn display_seats(seats:&Vec<Vec<Option<bool>>>)
{
    for x in 0..seats.len()
    {
        for y in 0..seats[x].len()
        {
            if seats[x][y] == Some(true)
            {
                print!("{}", '#');
            }
            else if seats[x][y] == Some(false)
            {
                print!("{}", 'L');
            }
            else 
            {
                print!("{}", '.');
            }
        }
        print!("\n");
    }
}

fn count_occupied(seats:&Vec<Vec<Option<bool>>>) -> u32
{
    let mut count = 0;
    for x in 0..seats.len()
    {
        for y in 0..seats[x].len()
        {
            if seats[x][y] == Some(true)
            {
                count += 1;
            }
        }
    }

    return count;
}

fn puzzle1(seats:&Vec<Vec<Option<bool>>>) -> u32
{
    let mut current_seats = seats.to_vec();
    loop
    {
        let mut new_seats = current_seats.to_vec();
        let changes = round(&current_seats, &mut new_seats);
        current_seats = new_seats;
        if changes == 0
        {
            break;
        }
    }

    return count_occupied(&current_seats);
}

fn round(old_seats:&Vec<Vec<Option<bool>>>, new_seats:&mut Vec<Vec<Option<bool>>>) -> u32
{
    let mut changes = 0;

    for x in 0..old_seats.len()
    {
        for y in 0..old_seats[x].len()
        {
            if old_seats[x][y] == None
            {
                continue;
            }

            let neighbours = count_neighbours(&old_seats, x, y);
            if old_seats[x][y] == Some(false) && neighbours == 0
            {
                new_seats[x][y] = Some(true);
                changes += 1;
            }
            else if old_seats[x][y] == Some(true) && neighbours >= 4
            {
                new_seats[x][y] = Some(false);
                changes += 1;
            }
        }
    }

    return changes;
}

fn count_neighbours(seats:&Vec<Vec<Option<bool>>>, x:usize, y:usize) -> u32
{
    let mut count = 0;

    let x_min = std::cmp::max(x as i32 - 1, 0);
    let x_max = std::cmp::min(x + 2, seats.len()) as i32;
    let y_min = std::cmp::max(y as i32 - 1, 0);
    let y_max = std::cmp::min(y + 2, seats[x].len()) as i32;

    for i in x_min..x_max
    {
        for j in y_min..y_max
        {
            if i == x as i32 && j == y as i32
            {
                continue;
            }
            if seats[i as usize][j as usize] != Some(true)
            {
                continue;
            }

            count += 1;
        }
    }

    return count;
}

fn puzzle2(seats:&Vec<Vec<Option<bool>>>) -> u32
{
    let mut current_seats = seats.to_vec();
    loop
    {
        let mut new_seats = current_seats.to_vec();
        let changes = round_tolerant(&current_seats, &mut new_seats);
        current_seats = new_seats;
        if changes == 0
        {
            break;
        }
    }

    display_seats(&current_seats);

    return count_occupied(&current_seats);
}

fn round_tolerant(old_seats:&Vec<Vec<Option<bool>>>, new_seats:&mut Vec<Vec<Option<bool>>>) -> u32
{
    let mut changes = 0;

    for x in 0..old_seats.len()
    {
        for y in 0..old_seats[x].len()
        {
            if old_seats[x][y] == None
            {
                continue;
            }

            let neighbours = count_directions(&old_seats, x, y);
            if old_seats[x][y] == Some(false) && neighbours == 0
            {
                new_seats[x][y] = Some(true);
                changes += 1;
            }
            else if old_seats[x][y] == Some(true) && neighbours >= 5
            {
                new_seats[x][y] = Some(false);
                changes += 1;
            }
        }
    }

    return changes;
}

fn count_directions(seats:&Vec<Vec<Option<bool>>>, x:usize, y:usize) -> u32
{
    let mut count = 0;

    for x_dir in -1..2
    {
        for y_dir in -1..2
        {
            if x_dir == 0 && y_dir == 0
            {
                continue;
            }

            count += check_direction(&seats, x as i32, y as i32, x_dir, y_dir) as u32;
        }
    }

    return count;
}

fn check_direction(seats:&Vec<Vec<Option<bool>>>, x_start:i32, y_start:i32, x_offset:i32, y_offset:i32) -> bool
{
    let mut x = x_start;
    let mut y = y_start;
    loop
    {
        x += x_offset;
        y += y_offset;

        if x < 0 || x >= seats.len() as i32 || y < 0 || y >= seats[x as usize].len() as i32
        {
            return false;
        }
        if seats[x as usize][y as usize] == Some(true)
        {
            return true;
        }
        else if seats[x as usize][y as usize] == Some(false)
        {
            return false;
        }
    }
}
