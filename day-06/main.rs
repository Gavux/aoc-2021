fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let input = include_str!("input.txt");

    let mut fish = input.split(',').map(| x | x.parse::<i32>().expect("Could not parse number")).collect::<Vec<i32>>();

    for _ in 0..18 {
        for j in 0..fish.len() {
            if fish[j] == 0 {
                fish[j] = 6;
                fish.push(8);
            } else {
                fish[j] -= 1;
            }
        }
    }

    println!("{}", fish.len());
}

fn part_b() {
    let input = include_str!("input.txt");

    let mut timers: Vec<u128> = vec![0; 9];
    let split_input = input.split(',').map(| x | x.parse::<u8>().expect("Could not parse number")).collect::<Vec<u8>>();
    for num in split_input {
        timers[num as usize] += 1;
    }

    for _ in 0..256 {
        let new_fish = timers.remove(0);
        timers.push(new_fish);
        timers[6] += new_fish;
    }

    println!("{}", timers.iter().sum::<u128>());
}