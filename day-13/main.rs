fn main() {
    part_a_b();
}

fn part_a_b() {
    let input = include_str!("input.txt");
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut points: Vec<(u32, u32)> = Vec::new();
    for &line in &split[0].split('\n').collect::<Vec<&str>>() {
        let split_p: Vec<u32> = line.split(',').map(| x | x.parse::<u32>().expect("Could not parse number.")).collect();
        points.push((split_p[0], split_p[1]));
    }

    let mut first: i32 = -1;
    for &line in &split[1].split('\n').collect::<Vec<&str>>() {
        let split_p: Vec<&str> = line.split("fold along ").collect::<Vec<&str>>()[1].split("=").collect();
        let coord = split_p[1].parse::<u32>().expect("Could not parse number.");
        if split_p[0] == "x" {
            for i in 0..points.len() {
                if points[i].0 > coord {
                    points[i].0 -= (points[i].0 - coord) * 2;
                }
            }
        }

        if split_p[0] == "y" {
            for i in 0..points.len() {
                if points[i].1 > coord {
                    points[i].1 -= (points[i].1 - coord) * 2;
                }
            }
        }

        points.sort_unstable();
        points.dedup();

        if first == -1 {
            first = points.len() as i32;
        }
    }

    println!("part a: {}", first);

    let mut max_x = points[0].0;
    let mut max_y = points[0].1;

    for &point in &points {
        if point.0 > max_x {
            max_x = point.0;
        }

        if point.1 > max_y {
            max_y = point.1;
        }
    }

    let mut display = vec![vec!["."; max_x as usize + 1]; max_y as usize + 1];
    for &point in &points {
        display[point.1 as usize][point.0 as usize] = "#";
    }

    println!("part b:");
    for i in 0..display.len() {
        for j in 0..display[i].len() {
            print!("{}", display[i][j]);
        }

        print!("\n");
    }
}