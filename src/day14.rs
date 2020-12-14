use std::collections::HashMap;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Op
{
    Mask,
    Mem
}

#[derive(Copy, Clone)]
struct Instruction
{
    op: Op,
    arg1: u64,
    arg2: u64
}

pub fn solve(contents:&String) -> (u64, u64)
{
    let mut instructions:Vec<Instruction> = Vec::new();
    let mut floating_instructions:Vec<Instruction> = Vec::new();

    for line in contents.lines()
    {
        if line.starts_with("mask")
        {
            let parts:Vec<&str> = line.split(" = ").collect();
            let mask:Vec<char> = parts[1].chars().collect();

            let mut and_mask = 0;
            let mut or_mask = 0;
            let mut base_mask = 0;
            let mut floating_mask = 0;
            for i in 0..mask.len()
            {
                let pos = mask.len() - 1 - i;
                let c = mask[i];
                match c
                {
                    'X' =>
                    {
                        and_mask |= 1 << pos;
                        floating_mask |= 1 << pos;
                    },
                    '1' =>
                    {
                        or_mask |= 1 << pos;
                        base_mask |= 1 << pos;
                    },
                    '0' => {},
                    _ => println!("C is something else")
                }
            }

            let ins = Instruction
            {
                op: Op::Mask,
                arg1: and_mask,
                arg2: or_mask
            };
            instructions.push(ins);

            let ins2 = Instruction
            {
                op: Op::Mask,
                arg1: base_mask,
                arg2: floating_mask
            };
            floating_instructions.push(ins2);
        }
        else
        {
            let start = line.find('[').unwrap() + 1;
            let end = line.find(']').unwrap();
            let location = &line[start..end];

            let parts:Vec<&str> = line.split(" = ").collect();
            let value:u64 = parts[1].parse().unwrap();

            let ins = Instruction
            {
                op: Op::Mem,
                arg1: location.parse().unwrap(),
                arg2: value
            };
            instructions.push(ins);
            floating_instructions.push(ins);
        }
    }

    return (puzzle1(&instructions), puzzle2(&floating_instructions));
}

fn puzzle1(instructions:&Vec<Instruction>) -> u64
{
    let mut memory:HashMap<u64, u64> = HashMap::new();

    let mut and_mask = 0;
    let mut or_mask = 0;

    for ins in instructions
    {
        if ins.op == Op::Mask
        {
            and_mask = ins.arg1;
            or_mask = ins.arg2;
        }
        else
        {
            let location = ins.arg1;
            let value = (ins.arg2 & and_mask) | or_mask;

            memory.insert(location, value);
        }
    }

    let mut result = 0;
    for pair in memory
    {
        result += pair.1;
    }

    return result;
}

fn puzzle2(instructions:&Vec<Instruction>) -> u64
{
    let mut memory:HashMap<u64, u64> = HashMap::new();

    let mut or_mask = 0;
    let mut floating_mask = 0;

    for ins in instructions
    {
        match ins.op
        {
            Op::Mask =>
            {
                or_mask = ins.arg1;
                floating_mask = ins.arg2;
            },
            Op::Mem =>
            {
                let mut locations:Vec<u64> = Vec::new();
                
                locations.push(ins.arg1 | or_mask);
                for i in 0..36
                {
                    let mask = 1 << i;
                    if (mask & floating_mask) == mask
                    {
                        for l in (0..locations.len()).rev()
                        {
                            let mutation = locations[l] ^ mask;
                            locations.push(mutation);
                        }
                    }
                }

                for loc in locations
                {
                    memory.insert(loc, ins.arg2);
                }
            }
        }
    }

    let mut result = 0;
    for pair in memory
    {
        result += pair.1;
    }

    return result;
}
