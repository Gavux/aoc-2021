use std::collections::HashMap;

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let input = include_str!("input.txt");
    let lines = input.split('\n');
    
    let mut count = 0;
    for line in lines {
        let split_input = line.split(" | ").collect::<Vec<&str>>();
        let _signal_patterns = split_input[0].split_whitespace().collect::<Vec<&str>>();
        let output_value = split_input[1].split_whitespace().collect::<Vec<&str>>();

        for value in output_value {
            let len = value.len();
            if len == 7 || len == 4 || len == 3 || len == 2 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

// TODO: Work on better variable naming.
fn part_b() {
    let orders: Vec<&str> = vec![
        "abcefg",   // 0
        "cf",       // 1
        "acdeg",    // 2
        "acdfg",    // 3
        "bcdf",     // 4
        "abdfg",    // 5
        "abdefg",   // 6
        "acf",      // 7
        "abcdefg",  // 8
        "abcdfg",   // 9
    ];

    let mut order_counts: HashMap<char, u8> = HashMap::new();
    for &order in &orders {
        for c in order.chars() {
            let count = match order_counts.get(&c) {
                Some(k) => k.clone(),
                _ => 0,
            };
            order_counts.insert(c, count + 1);
        }
    }

    let mut order_sums: Vec<u8> = Vec::new();
    for &order in &orders {
        let mut sum = 0;
        for c in order.chars() {
            sum += order_counts.get(&c).unwrap();
        }
        order_sums.push(sum);
    }

    let input = include_str!("input.txt");
    let lines = input.split('\n');
    
    let mut count = 0;
    for line in lines {
        let split_input = line.split(" | ").collect::<Vec<&str>>();
        let sequence_patterns = split_input[0].split_whitespace().collect::<Vec<&str>>();
        let output_value = split_input[1].split_whitespace().collect::<Vec<&str>>();

        let mut numbers = vec![String::from(""); 10];
        let mut counts: HashMap<char, u8> = HashMap::new();
        for &pattern in &sequence_patterns {
            for c in pattern.chars() {
                let count = match counts.get(&c) {
                    Some(k) => k.clone(),
                    _ => 0,
                };

                counts.insert(c, count + 1);
            }
        }

        for &pattern in &sequence_patterns {
            let mut sum = 0;
            for c in pattern.chars() {
                sum += counts.get(&c).unwrap();
            }

            for i in 0..order_sums.len() {
                if order_sums[i] == sum {
                    let mut chars = pattern.split("").collect::<Vec<&str>>();
                    chars.sort();
                    numbers[i] = chars.join("");
                }
            }
        }

        let mut num = String::new();
        for value in output_value {
            for i in 0..numbers.len() {
                let mut chars = value.split("").collect::<Vec<&str>>();
                chars.sort();
                if numbers[i] == chars.join("") {
                    num += &i.to_string();
                }
            }
        }

        count += num.parse::<u32>().unwrap();
    }

    println!("{}", count);
}