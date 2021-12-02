use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file");

    let lines: Vec<&str> = contents.split('\n').collect();
    let mut threes: Vec<i32> = Vec::default();
    let mut prev = -1;
    let mut count = 0;
    for line in lines.iter() {
        let num = line.parse::<i32>().unwrap();
        if threes.len() == 3 {
            let mut sum = 0;
            for l in threes.iter() {
                sum += l;
            }

            if prev != -1 && sum > prev {
                count += 1;
            }

            prev = sum;

            threes.remove(0);
        }

        threes.push(num);
    }

    println!("Read {} lines", lines.len());

    println!("{}", count);
}