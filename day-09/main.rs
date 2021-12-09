fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let input = include_str!("input.txt");
    let lines: Vec<&str> = input.split("\n").collect();
    let mut field: Vec<Vec<u32>> = Vec::new();
    for &line in &lines {
        let split: Vec<u32> = line.chars().map(| x |
            x.to_digit(10).unwrap()
        ).collect();
        field.push(split);
    }

    let mut sum = 0;
    for y in 0..field.len() {
        for x in 0..field[y].len() {
            let mut smallest = true;
            let current = field[y][x];
            if y > 0 && field[y - 1][x] <= current {
                smallest = false;
            }

            if x > 0 && field[y][x - 1] <= current {
                smallest = false;
            }

            if y < field.len() - 1 && field[y + 1][x] <= current {
                smallest = false;
            }

            if x < field[y].len() - 1 && field[y][x + 1] <= current {
                smallest = false;
            }

            if smallest {
                sum += current + 1;
            }
        }
    }

    println!("{}", sum);
}

fn part_b() {
    let input = include_str!("input.txt");
    let lines: Vec<&str> = input.split("\n").collect();
    let mut field: Vec<Vec<u32>> = Vec::new();
    for &line in &lines {
        let split: Vec<u32> = line.chars().map(| x |
            x.to_digit(10).unwrap()
        ).collect();
        field.push(split);
    }

    let mut basin_sizes: Vec<u32> = Vec::new();
    for y in 0..field.len() {
        for x in 0..field[y].len() {
            let mut smallest = true;
            let current = field[y][x];
            if y > 0 && field[y - 1][x] <= current {
                smallest = false;
            }

            if x > 0 && field[y][x - 1] <= current {
                smallest = false;
            }

            if y < field.len() - 1 && field[y + 1][x] <= current {
                smallest = false;
            }

            if x < field[y].len() - 1 && field[y][x + 1] <= current {
                smallest = false;
            }

            if smallest {
                let mut basins = Vec::new();
                find_basin(&field, x, y, &mut basins);
                basin_sizes.push(basins.len() as u32);
            }
        }
    }

    basin_sizes.sort();
    basin_sizes.reverse();

    println!("{}", basin_sizes.drain(0..=2).reduce(| x, y | x * y).unwrap());
}

fn find_basin(field: &Vec<Vec<u32>>, x: usize, y: usize, points: &mut Vec<(usize, usize)>) {
    if points.contains(&(x, y)) || y >= field.len() || x >= field[y].len() || field[y][x] == 9 {
        return;
    }

    points.push((x, y));

    if x > 0 {
        find_basin(field, x - 1, y, points);
    }

    if y > 0 {
        find_basin(field, x, y - 1, points);
    }

    find_basin(field, x, y + 1, points);
    find_basin(field, x + 1, y, points);

}