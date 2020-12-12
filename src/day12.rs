struct Instruction
{
    cmd: char,
    amount: i64
}

pub fn solve(contents:&String) -> (i64, i64)
{
    let mut instructions:Vec<Instruction> = Vec::new();

    for line in contents.lines()
    {
        let cmd = line.chars().nth(0).unwrap();
        let amount:i64 = line[1..].parse().unwrap();

        let instruction = Instruction
        {
            cmd: cmd,
            amount: amount
        };

        instructions.push(instruction);
    }

    return (puzzle1(&instructions), puzzle2(&instructions));
}

fn puzzle1(instructions:&Vec<Instruction>) -> i64
{
    let mut x = 0;
    let mut y = 0;
    let mut x_dir = 1;
    let mut y_dir = 0;
    let mut rotation = 0;

    for ins in instructions
    {
        match ins.cmd
        {
            'N' => y -= ins.amount,
            'S' => y += ins.amount,
            'W' => x -= ins.amount,
            'E' => x += ins.amount,
            'F' => {
                x += x_dir * ins.amount;
                y += y_dir * ins.amount;
            },
            'L' => {
                rotation -= ins.amount;
                x_dir = deg_to_rad(rotation).cos().round() as i64;
                y_dir = deg_to_rad(rotation).sin().round() as i64;
            },
            'R' => {
                rotation += ins.amount;
                x_dir = deg_to_rad(rotation).cos().round() as i64;
                y_dir = deg_to_rad(rotation).sin().round() as i64;
            }
            _ => println!("Nothing to see here")
        }
    }

    return x.abs() + y.abs();
}

fn puzzle2(instructions:&Vec<Instruction>) -> i64
{
    let mut x = 0;
    let mut y = 0;
    let mut way_x = 10;
    let mut way_y = -1;

    for ins in instructions
    {
        match ins.cmd
        {
            'N' => way_y -= ins.amount,
            'S' => way_y += ins.amount,
            'W' => way_x -= ins.amount,
            'E' => way_x += ins.amount,
            'F' => {
                x += way_x * ins.amount;
                y += way_y * ins.amount;
            },
            'L' => {
                let d = f64::sqrt(way_x as f64 * way_x as f64 + way_y as f64 * way_y as f64);
                let rotation = f64::atan2(way_y as f64, way_x as f64) - deg_to_rad(ins.amount);
                way_x = (rotation.cos() * d).round() as i64;
                way_y = (rotation.sin() * d).round() as i64;
            },
            'R' => {
                let d = f64::sqrt(way_x as f64 * way_x as f64 + way_y as f64 * way_y as f64);
                let rotation = f64::atan2(way_y as f64, way_x as f64) + deg_to_rad(ins.amount);
                way_x = (rotation.cos() * d).round() as i64;
                way_y = (rotation.sin() * d).round() as i64;
            },
            _ => println!("Nothing to see here")
        }
    }

    return x.abs() + y.abs();
}

fn deg_to_rad(deg:i64) -> f64
{
    return (deg as f64) * (std::f64::consts::PI / 180.0);
}
