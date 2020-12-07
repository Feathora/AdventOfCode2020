use std::collections::HashMap;

struct Bag
{
    colour:String,
    bags:HashMap<String, u32>
}

pub fn solve(contents:&String) -> (u32, u32)
{
    let mut root_bags:Vec<Bag> = Vec::new();

    for rule in contents.lines()
    {
        let mut rule_parts = rule.split("contain");
        let mut containing_bag_parts = rule_parts.nth(0).expect("No first part").split_whitespace();
        let containing_bag_colour = format!("{} {}", containing_bag_parts.nth(0).expect("No first colour part"), containing_bag_parts.nth(0).expect("No second colour part"));

        let mut bag = Bag
        {
            colour: containing_bag_colour.to_string(),
            bags: HashMap::new()
        };

        let contained_rule_part = rule_parts.nth(0).expect("No second rule part");
        if contained_rule_part.contains("no other bags")
        {
            root_bags.push(bag);
            continue;
        }

        let contained_bags:Vec<&str> = contained_rule_part.split(',').collect();
        for contained_bag in contained_bags
        {
            let contained_bag_parts:Vec<&str> = contained_bag.split_whitespace().collect();
            let contained_bag_colour = format!("{} {}", contained_bag_parts[1], contained_bag_parts[2]);
            let contained_bag_count:u32 = contained_bag_parts[0].parse().unwrap();

            bag.bags.insert(contained_bag_colour, contained_bag_count);
        }

        root_bags.push(bag);
    }

    return (puzzle1(&root_bags), puzzle2(&root_bags));
}

fn puzzle1(bags:&Vec<Bag>) -> u32
{
    let mut count = 0;

    for bag in bags
    {
        count += contains_shiny_gold(bags, bag) as u32;
    }

    return count;
}

fn contains_shiny_gold(bags:&Vec<Bag>, bag:&Bag) -> bool
{
    for other in &bag.bags
    {
        if other.0.contains("shiny gold")
        {
            return true;
        } 
        else if contains_shiny_gold(bags, bags.iter().filter(|b| b.colour.contains(other.0)).nth(0).expect(&format!("No {} there", other.0).to_string()))
        {
            return true;
        }
    }

    return false;
}

fn puzzle2(bags:&Vec<Bag>) -> u32
{
    for bag in bags
    {
        if !bag.colour.contains("shiny gold")
        {
            continue;
        }

        return count_bags_inside(bags, bag);
    }

    return 0;
}

fn count_bags_inside(bags:&Vec<Bag>, bag:&Bag) -> u32
{
    let mut count = 0;

    for (key, value) in &bag.bags
    {
        let bag = bags.iter().filter(|b| b.colour.contains(key)).nth(0).expect(&format!("No {} there", key).to_string());
        count += (1 + count_bags_inside(bags, bag)) * value;
    }

    return count;
}
