use std::ops::RangeInclusive;
use std::collections::HashMap;

#[derive(Clone)]
struct Ticket
{
    fields:Vec<u64>
}

struct Field
{
    name:String,
    ranges:Vec<RangeInclusive<u64>>
}

pub fn solve(contents:&String) -> (u64, u64)
{
    let content_parts:Vec<&str> = contents.split("\n\n").collect();
    let fields_part = content_parts[0];
    let your_part = content_parts[1];
    let others_part = content_parts[2];

    let mut fields:Vec<Field> = Vec::new();

    for content_line in fields_part.lines()
    {
        let field_name = content_line.split(':').nth(0).unwrap();
        let ranges_parts:Vec<&str> = content_line.split(": ").nth(1).unwrap().split(" or ").collect();

        let mut ranges:Vec<RangeInclusive<u64>> = Vec::new();

        for range_parts in ranges_parts
        {
            let min:u64 = range_parts.split('-').nth(0).unwrap().parse().unwrap();
            let max:u64 = range_parts.split('-').nth(1).unwrap().parse().unwrap();

            let range = min..=max;
            ranges.push(range);
        }

        let field = Field
        {
            name: String::from(field_name),
            ranges: ranges
        };

        fields.push(field);
    }

    let mut your_fields:Vec<u64> = Vec::new();
    for field in your_part.lines().nth(1).unwrap().split(',')
    {
        your_fields.push(field.parse().unwrap());
    }

    let your_ticket = Ticket
    {
        fields: your_fields
    };

    let mut other_tickets:Vec<Ticket> = Vec::new();
    for l in 1..others_part.lines().count()
    {
        let mut other_fields:Vec<u64> = Vec::new();
        let line = others_part.lines().nth(l).unwrap();
        for field in line.split(',')
        {
            other_fields.push(field.parse().unwrap());
        }

        let other_ticket = Ticket
        {
            fields: other_fields
        };
        other_tickets.push(other_ticket);
    }

    return (puzzle1(&fields, &other_tickets), puzzle2(&fields, &other_tickets, &your_ticket));
}

fn puzzle1(fields:&Vec<Field>, tickets:&Vec<Ticket>) -> u64
{
    let mut error_range = 0;

    for ticket in tickets
    {
        'ticket_fields: for ticket_field in &ticket.fields
        {
            for check_field in fields
            {
                for range in &check_field.ranges
                {
                    if range.contains(&ticket_field)
                    {
                        continue 'ticket_fields;
                    }
                }
            }

            error_range += ticket_field;
        }
    }

    return error_range;
}

fn puzzle2(fields:&Vec<Field>, tickets:&Vec<Ticket>, your_ticket:&Ticket) -> u64
{
    let mut valid_tickets:Vec<Ticket> = vec![your_ticket.clone()];

    for ticket in tickets
    {
        if is_valid(fields, ticket)
        {
            valid_tickets.push(ticket.clone());
        }
    }

    let mut indices:HashMap<u64, u64> = HashMap::new();

    'outer_loop: loop
    {
        'field_index: for field_index in 0..fields.len()
        {
            let mut valid = vec![true; fields.len()];
            for index in &indices
            {
                valid[*index.0 as usize] = false;
            }
            for ticket in &valid_tickets
            {
                for f in 0..fields.len()
                {
                    let mut v = false;
                    for range in &fields[f].ranges
                    {
                        v |= range.contains(&ticket.fields[field_index]);
                    }
                    if !v
                    {
                        valid[f] = false;
                    }
                }
            }
            let valid_count = valid.iter().filter(|v| **v).count();

            if valid_count == 1
            {
                for i in 0..valid.len()
                {
                    if valid[i]
                    {
                        indices.insert(i as u64, field_index as u64);
                        
                        if indices.len() == fields.len()
                        {
                            break 'outer_loop;
                        }
                        continue 'field_index;
                    }
                }
            }
        }
    }

    let mut result = 1;
    for index in indices
    {
        if fields[index.0 as usize].name.starts_with("departure")
        {
            result *= your_ticket.fields[index.1 as usize];
        }
    }

    return result;
}

fn is_valid(fields:&Vec<Field>, ticket:&Ticket) -> bool
{
    'ticket_fields: for ticket_field in &ticket.fields
    {
        for check_field in fields
        {
            for range in &check_field.ranges
            {
                if range.contains(&ticket_field)
                {
                    continue 'ticket_fields;
                }
            }
        }

        return false;
    }

    return true;
}
