use std::collections::HashMap;

type Dimension = HashMap<i64, HashMap<i64, HashMap<i64, HashMap<i64, bool>>>>;

pub fn solve(contents:&String) -> (u64, u64)
{
    let rounds = 6;
    let depth = 1;
    let height = contents.lines().count() as i64;
    let width = contents.lines().nth(0).unwrap().len() as i64;

    let mut dimensions:Dimension = Dimension::new();
    let mut z_map:HashMap<i64, HashMap<i64, HashMap<i64, bool>>> = HashMap::new();

    let mut y_map:HashMap<i64, HashMap<i64, bool>> = HashMap::new();
    for l in 0..contents.lines().count()
    {
        let mut x_map:HashMap<i64, bool> = HashMap::new();

        let line = contents.lines().nth(l).unwrap();
        for c in 0..line.len()
        {
            let chr = line.chars().nth(c).unwrap();
            if chr == '.'
            {
                x_map.insert(c as i64, false);
            }
            else if chr == '#'
            {
                x_map.insert(c as i64, true);
            }
        }

        y_map.insert(l as i64, x_map);
    }

    z_map.insert(0, y_map);
    dimensions.insert(0, z_map);

    for w_padding in -rounds..dimensions.len() as i64 + rounds
    {
        if !dimensions.contains_key(&w_padding)
        {
            dimensions.insert(w_padding, HashMap::new());
        }

        let z_map = dimensions.get_mut(&w_padding).unwrap();
        for z_padding in -rounds..depth + rounds
        {
            if !z_map.contains_key(&z_padding)
            {
                z_map.insert(z_padding, HashMap::new());
            }

            let y_map = z_map.get_mut(&z_padding).unwrap();
            for y_padding in -rounds..height + rounds
            {
                if !y_map.contains_key(&y_padding)
                {
                    y_map.insert(y_padding, HashMap::new());
                }

                let x_map = y_map.get_mut(&y_padding).unwrap();
                for x_padding in -rounds..width + rounds
                {
                    if !x_map.contains_key(&x_padding)
                    {
                        x_map.insert(x_padding, false);
                    }
                }
            }
        }
    }

    return (puzzle1(), puzzle2(&mut dimensions));
}

fn puzzle1() -> u64
{
    return 0;
}

fn puzzle2(dimensions:&mut Dimension) -> u64
{
    for _ in 0..6
    {
        round(dimensions);
    }

    let mut active = 0;

    for (_w_index, z_map) in dimensions.iter()
    {
        for (_z_index, y_map) in z_map.iter()
        {
            for (_y_index, x_map) in y_map.iter()
            {
                for (_x_index, value) in x_map.iter()
                {
                    if *value
                    {
                        active += 1;
                    }
                }
            }
        }
    }

    return active;
}

fn round(dimension:&mut Dimension)
{
    let old = dimension.clone();

    for (w_index, z_map) in dimension.iter_mut()
    {
        for (z_index, y_map) in z_map.iter_mut()
        {
            for (y_index, x_map) in y_map.iter_mut()
            {
                for (x_index, value) in x_map.iter_mut()
                {
                    let neighbours = count_neighbours(&old, *x_index, *y_index, *z_index, *w_index);
                    if *value && neighbours != 2 && neighbours != 3
                    {
                        *value = false;
                    }
                    else if !*value && neighbours == 3
                    {
                        *value = true;
                    }
                }
            }
        }
    }
}

fn count_neighbours(dimensions:&Dimension, x:i64, y:i64, z:i64, w:i64) -> u64
{
    let x_min = x - 1;
    let x_max = x + 1;
    let y_min = y - 1;
    let y_max = y + 1;
    let z_min = z - 1;
    let z_max = z + 1;
    let w_min = w - 1;
    let w_max = w + 1;

    let mut count = 0;

    for w_index in w_min..=w_max
    {
        if !dimensions.contains_key(&w_index)
        {
            continue;
        }

        let z_map = dimensions.get(&w_index).unwrap();
        for z_index in z_min..=z_max
        {
            if !z_map.contains_key(&z_index)
            {
                continue;
            }

            let y_map = z_map.get(&z_index).unwrap();
            for y_index in y_min..=y_max
            {
                if !y_map.contains_key(&y_index)
                {
                    continue;
                }

                let x_map = y_map.get(&y_index).unwrap();
                for x_index in x_min..=x_max
                {
                    if w_index == w && z_index == z && y_index == y && x_index == x
                    {
                        continue;
                    }
                    if !x_map.contains_key(&x_index)
                    {
                        continue;
                    }

                    let value = x_map.get(&x_index).unwrap();
                    if *value
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    return count;
}
