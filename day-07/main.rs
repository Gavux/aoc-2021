fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let input = include_str!("input.txt");
    let positions = input.split(',').map(| x | x.parse::<i32>().expect("Could not parse as number")).collect::<Vec<i32>>();
    let start = positions.iter().min().unwrap();
    let end = positions.iter().max().unwrap();
    let mut fuel_needed = vec![0; (*end - *start + 1) as usize];
    for i in *start..=*end {
        for pos in &positions {
            fuel_needed[(i - start) as usize] += (pos - i).abs();
        }
    }

    println!("{}", fuel_needed.iter().min().unwrap());
}

fn part_b() {
    let input = include_str!("input.txt");
    let positions = input.split(',').map(| x | x.parse::<i32>().expect("Could not parse as number")).collect::<Vec<i32>>();
    let start = positions.iter().min().unwrap();
    let end = positions.iter().max().unwrap();
    let mut fuel_needed = vec![0; (*end - *start + 1) as usize];
    for i in *start..=*end {
        for pos in &positions {
            let step = (pos - i).abs();
            fuel_needed[(i - start) as usize] += (0.5 * step as f32 * (step as f32 + 1.0)) as i32;
        }
    }

    println!("{}", fuel_needed.iter().min().unwrap());
}