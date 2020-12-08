type Operation = fn(i32, &mut i32, &mut i32);

struct Instruction
{
    operation: Operation,
    argument: i32
}

#[derive(Debug)]
enum RunError
{
    InfiniteLoop
}

pub fn solve(contents:&String) -> (u32, u32)
{
    let mut instructions:Vec<Instruction> = Vec::new();

    for line in contents.lines()
    {
        let parts:Vec<&str> = line.split_whitespace().collect();
        let op:Operation = match parts[0]
        {
            "acc" => acc,
            "jmp" => jmp,
            "nop" => nop,
            _ => panic!("No valid operation")
        };
        let arg:i32 = parts[1].parse().unwrap();

        let instruction = Instruction
        {
            operation: op,
            argument: arg
        };

        instructions.push(instruction);
    }

    return (puzzle1(&instructions), puzzle2(&instructions));
}

fn acc(argument:i32, accumulator:&mut i32, offset:&mut i32)
{
    *accumulator += argument;
    *offset += 1;
}

fn jmp(argument:i32, _accumulator:&mut i32, offset:&mut i32)
{
    *offset += argument;
}

fn nop(_argument:i32, _accumulator:&mut i32, offset:&mut i32)
{
    *offset += 1;
}

fn puzzle1(instructions:&Vec<Instruction>) -> u32
{
    let mut accumulator = 0;
    let mut offset = 0;
    let mut visited = vec![false; instructions.len()];
    loop
    {
        if visited[offset as usize]
        {
            break;
        }

        visited[offset as usize] = true;

        let instruction = &instructions[offset as usize];
        let op = instruction.operation;
        op(instruction.argument, &mut accumulator, &mut offset);
    }

    return accumulator as u32;
}

fn puzzle2(instructions:&Vec<Instruction>) -> u32
{
    for i in 0..instructions.len()
    {
        let op = instructions[i].operation;
        if op as usize == acc as usize
        {
            continue;
        }

        let result = run(instructions, i);
        if result.is_ok()
        {
            return result.unwrap();
        }
    }

    return 0;
}

fn run(instructions:&Vec<Instruction>, flip_index:usize) -> Result<u32, RunError>
{
    let mut accumulator = 0;
    let mut offset:i32 = 0;
    let mut visited = vec![false; instructions.len()];
    while (offset as usize) < instructions.len()
    {
        if visited[offset as usize]
        {
            return Err(RunError::InfiniteLoop);
        }

        visited[offset as usize] = true;

        let instruction = &instructions[offset as usize];
        let mut op:Operation = instruction.operation;

        if flip_index == offset as usize
        {
            if op as usize == jmp as usize
            {
                op = nop;
            }
            else if op as usize == nop as usize
            {
                op = jmp;
            }
        }

        op(instruction.argument, &mut accumulator, &mut offset);
    }

    return Ok(accumulator as u32);
}
