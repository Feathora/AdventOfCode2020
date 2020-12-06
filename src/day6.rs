pub fn solve(contents:&String) -> (u32, u32)
{
    let groups:Vec<&str> = contents.split("\n\n").collect();

    return (puzzle1(&groups), puzzle2(&groups));
}

fn puzzle1(groups:&Vec<&str>) -> u32
{
    let mut count = 0;

    for group in groups
    {
        let mut answers = [false; 26];
        for person in group.lines()
        {
            for a in person.chars()
            {
                let i = (a as usize) - ('a' as usize);
                answers[i] = true;
            }
        }

        count += answers.iter().filter(|a| **a).count();
    }

    return count as u32;
}

fn puzzle2(groups:&Vec<&str>) -> u32
{
    let mut count = 0;

    for group in groups
    {
        let mut answers = [0; 26];
        for person in group.lines()
        {
            for a in person.chars()
            {
                let i = (a as usize) - ('a' as usize);
                answers[i] += 1;
            }
        }

        count += answers.iter().filter(|a| **a == group.lines().count()).count();
    }

    return count as u32;
}
