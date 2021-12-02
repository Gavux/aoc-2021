use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file");

    let lines: Vec<&str> = contents.split('\n').collect();
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for line in lines.iter() {
        let split_line: Vec<&str> = line.split(' ').collect();
        let nu = split_line.get(1).expect("Could not get number").parse::<i32>().expect("Could not parse as number");
        match *split_line.get(0).expect("Could not get command") {
            "forward" => {
                x += nu;
                y += aim * nu;
            },
            "up" => aim -= nu,
            "down" => aim += nu,
            &_ => {}
        }
    }

    println!("depth: {}, horizontal: {}, multiplied: {}", x, y, (x * y))
}