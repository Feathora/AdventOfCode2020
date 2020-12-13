struct Bus
{
    id: u64,
    offset: u64
}

pub fn solve(contents:&String) -> (u64, u64)
{
    let timestamp:u64 = contents.lines().nth(0).unwrap().parse().unwrap();
    let buses_str:Vec<&str> = contents.lines().nth(1).unwrap().split(',').collect();
    
    let mut buses:Vec<Bus> = Vec::new();
    let mut offset = 0;
    for bus_str in buses_str
    {
        if bus_str != "x"
        {
            let bus = Bus
            {
                id: bus_str.parse().unwrap(),
                offset: offset
            };

            buses.push(bus);
        }

        offset += 1;
    }

    buses.sort_by(|a, b| b.id.cmp(&a.id));

    return (puzzle1(timestamp, &buses), puzzle2(&buses));
}

fn puzzle1(timestamp:u64, buses:&Vec<Bus>) -> u64
{
    let mut first_bus_id = 0;
    let mut first_time = u64::MAX;

    for bus in buses
    {
        let times_went = timestamp / bus.id;
        let next = (times_went + 1) * bus.id;
        let wait_time = next - timestamp;

        if wait_time < first_time
        {
            first_time = wait_time;
            first_bus_id = bus.id;
        }
    }

    return first_bus_id * first_time;
}

fn puzzle2(buses:&Vec<Bus>) -> u64
{
    let step_size = buses[0].id;
    let went = 100000000000000u64 / step_size;
    let mut timestamp = ((went + 1) * step_size) - buses[0].offset;

    loop
    {
        let mut found = true;
        for bus in buses
        {
            if (timestamp + bus.offset) % bus.id != 0
            {
                found = false;
                break;
            }
        }

        if found { break; }

        timestamp += step_size;
    }

    return timestamp;
}
