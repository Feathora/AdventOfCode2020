use eval::eval;

pub fn solve(contents:&String) -> (u64, u64)
{
    let lines:Vec<&str> = contents.lines().collect();

    return (puzzle1(&lines), puzzle2(&lines));
}

fn puzzle1(expressions:&Vec<&str>) -> u64
{
    let mut result = 0;

    for expression in expressions
    {
        let mut expr = String::from(*expression);
        resolve(&mut expr, 0, calculate1);
        result += calculate1(&expr);
    }

    return result;
}

fn puzzle2(expressions:&Vec<&str>) -> u64
{
    let mut result = 0;

    for expression in expressions
    {
        let mut expr = String::from(*expression);
        resolve(&mut expr, 0, calculate2);
        result += calculate2(&expr);
    }

    return result;
}

fn resolve(expression:&mut String, start_index:usize, calc:fn(&str) -> u64)
{
    let mut i = start_index;
    while i < expression.len()
    {
        let c = expression.chars().nth(i).unwrap();
        if c == '('
        {
            resolve(expression, i + 1, calc);
        }
        else if c == ')'
        {
            let sub = &expression[start_index..i];
            let result = calc(sub);
            expression.replace_range(start_index - 1..=i, &result.to_string());
            return;
        }

        i += 1;
    }
}

fn calculate1(expression:&str) -> u64
{
    let parts:Vec<&str> = expression.split_whitespace().collect();
    let mut lhs:u64 = parts[0].parse().unwrap();
    for i in (1..parts.len()).step_by(2)
    {
        let op = parts[i];
        let rhs:u64 = parts[i + 1].parse().unwrap();
        match op
        {
            "*" => lhs *= rhs,
            "+" => lhs += rhs,
            _ => {}
        }
    }
    return lhs;
}

fn calculate2(expression:&str) -> u64
{
    let mut parts:Vec<String> = expression.split_whitespace().map(|s| String::from(s)).collect();
    let mut i = 1;
    while i < parts.len()
    {
        if parts[i] == "+"
        {
            let lhs:u64 = parts[i - 1].parse().unwrap();
            let rhs:u64 = parts[i + 1].parse().unwrap();
            let r = (lhs + rhs).to_string();
            parts[i - 1] = r;
            parts.remove(i + 1);
            parts.remove(i);
        }
        else
        {
            i += 2;
        }
    }
    let expr:String = parts.join(" ");
    return eval(&expr).unwrap().as_u64().unwrap();
}
