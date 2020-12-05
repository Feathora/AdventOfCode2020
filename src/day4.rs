pub fn solve(contents:&String) -> (u32, u32)
{
    let passports:Vec<&str> = contents.split("\n\n").collect();
    
    let result1 = puzzle1(&passports);
    let result2 = puzzle2(&passports);

    (result1, result2)
}

fn puzzle1(passports:&Vec<&str>) -> u32
{
    let mut count = 0;

    for passport in passports
    {
        count += has_all_reqs(passport) as u32;
    }

    return count;
}

fn has_all_reqs(passport:&str) -> bool
{
    let reqs = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];

    for req in &reqs
    {
        if !String::from(passport).contains(req)
        {
            return false;
        }
    }

    return true;
}

fn puzzle2(passports:&Vec<&str>) -> u32
{
    let mut count = 0;

    'passports: for passport in passports
    {
        if !has_all_reqs(passport)
        {
            continue;
        }

        let dataline = String::from(*passport).replace("\n", " ");
        let parts = dataline.split_whitespace();
        for part in parts
        {
            let pair:Vec<&str> = part.split(':').collect();
            
            match pair[0]
            {
                "byr" => 
                {
                    let b:u32 = pair[1].parse().unwrap();
                    if b < 1920 || b > 2002
                    {
                        continue 'passports;
                    }
                },
                "iyr" =>
                {
                    let i:u32 = pair[1].parse().unwrap();
                    if i < 2010 || i > 2020
                    {
                        continue 'passports;
                    }
                },
                "eyr" =>
                {
                    let e:u32 = pair[1].parse().unwrap();
                    if e < 2020 || e > 2030
                    {
                        continue 'passports;
                    }
                },
                "hgt" =>
                {
                    let height_string = String::from(pair[1]);
                    let height:u32 = (&height_string[0..height_string.len() - 2]).parse().unwrap();
                    let height_type = &height_string[height_string.len() - 2..height_string.len()];
                    match height_type
                    {
                        "in" =>
                        {
                            if height < 59 || height > 76
                            {
                                continue 'passports;
                            }
                        },
                        "cm" =>
                        {
                            if height < 150 || height > 193
                            {
                                continue 'passports;
                            }
                        },
                        _ => continue 'passports
                    }
                },
                "hcl" =>
                {
                    if pair[1].len() != 7 || pair[1].chars().nth(0).unwrap() != '#'
                    {
                        continue 'passports;
                    }
                    let colour_str = &String::from(pair[1])[1..];
                    for c in colour_str.chars()
                    {
                        let valid = (c >= '0' && c <= '9') || (c >= 'A' && c <= 'F') || c >= 'a' && c <= 'f';
                        if !valid
                        {
                            continue 'passports;
                        }
                    }
                },
                "ecl" =>
                {
                    let valid = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                    if !valid.contains(&pair[1])
                    {
                        continue 'passports;
                    }
                },
                "pid" =>
                {
                    if pair[1].len() != 9
                    {
                        continue 'passports;
                    }
                    let parsed_pid:i32 = pair[1].parse().unwrap_or(-1);
                    if parsed_pid == -1
                    {
                        continue 'passports;
                    }
                },
                _ => continue,
            }
        }

        count += 1;
    }

    return count;
}
