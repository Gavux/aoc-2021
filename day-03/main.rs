use std::fs;

// Forgive me, lord.
fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.");

    let lines: Vec<&str>  = input.split('\n').collect();
    let mut count_ones: Vec<i32> = Vec::default();
    let mut count_zeros: Vec<i32> = Vec::default();
    for j in 0..lines.len() {
        let split_line: Vec<char> = lines[j].chars().collect();
        if j == 0 {
            count_ones = vec![0; split_line.len()];
            count_zeros = vec![0; split_line.len()];
        }
        for i in 0..split_line.len() {
            match split_line[i] {
                '1' => count_ones[i] += 1,
                '0' => count_zeros[i] += 1,
                _ => panic!("Empty.")
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..count_ones.len() {
        if count_ones[i] > count_zeros[i] {
            gamma += i32::pow(2, count_ones.len() as u32 - i as u32 - 1);
        } else {
            epsilon += i32::pow(2, count_ones.len() as u32 - i as u32 - 1);
        }
    }

    println!("{}", gamma * epsilon);
}

fn part_b() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.");
    let mut lines: Vec<&str> = input.split('\n').collect();

    let mut oxy_generator: u32 = 0;
    let iterations = lines[0].len();
    for i in 0..iterations {
        let mut zeros = Vec::default();
        let mut ones = Vec::default();
        for j in 0..lines.len() {
            let line_chars: Vec<char> = lines[j].chars().collect();
            match line_chars[i] {
                '0' => zeros.push(lines[j]),
                '1' => ones.push(lines[j]),
                _ => panic!("Empty."),
            }
        }

        if zeros.len() > ones.len() {
            lines = zeros;
        } else {
            lines = ones;
        }

        if lines.len() == 1 {
            oxy_generator = binary_string_as_number(lines[0]);
            break;
        }
    }

    lines = input.split('\n').collect();

    let mut co2_scrubber: u32 = 0;
    let iterations = lines[0].len();
    for i in 0..iterations {
        let mut zeros = Vec::default();
        let mut ones = Vec::default();
        for j in 0..lines.len() {
            let line_chars: Vec<char> = lines[j].chars().collect();
            match line_chars[i] {
                '0' => zeros.push(lines[j]),
                '1' => ones.push(lines[j]),
                _ => panic!("Empty."),
            }
        }

        if ones.len() < zeros.len() {
            lines = ones;
        } else {
            lines = zeros;
        }

        if lines.len() == 1 {
            co2_scrubber = binary_string_as_number(lines[0]);
            break;
        }
    }

    println!("{}", oxy_generator * co2_scrubber);
}

fn binary_string_as_number(input: &str) -> u32 {
    let mut result = 0;
    let input_chars: Vec<char> = input.chars().collect();
    for i in 0..input_chars.len() {
        if input_chars[i] == '1' {
            result += u32::pow(2, input_chars.len() as u32 - i as u32 - 1);
        }
    }

    result
}