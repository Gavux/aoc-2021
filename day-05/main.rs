fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let input = include_str!("input.txt");

    let lines: Vec<&str> = input.split('\n').collect();
    let mut blocks: Vec<Block> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in lines {
        let split_line: Vec<&str> = line.split(" -> ").collect();
        let p1 = Point::from(split_line[0]);
        let p2 = Point::from(split_line[1]);

        if p1.x > max_x {
            max_x = p1.x;
        }

        if p2.x > max_x {
            max_x = p2.x
        }

        if p1.y > max_y {
            max_y = p1.y
        }

        if p2.y > max_y {
            max_y = p2.y
        }

        if p1.x == p2.x || p1.y == p2.y {
            if p1.y > p2.y || p1.x > p2.x {
                blocks.push(Block { from: p2, to: p1 });
            } else {
                blocks.push(Block { from: p1, to: p2 });
            }
        }
    }

    let mut field = vec![vec![0; max_y as usize + 1]; max_x as usize + 1];
    for block in blocks {
        for x in block.from.x..block.to.x + 1 {
            for y in block.from.y..block.to.y + 1 {
                field[x as usize][y as usize] += 1;
            }
        }
    }

    let mut sum = 0;
    for x in 0..field.len() {
        for y in 0..field[x].len() {
            if field[x][y] > 1 {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}

fn part_b() {
    let input = include_str!("input.txt");

    let lines: Vec<&str> = input.split('\n').collect();
    let mut blocks: Vec<Block> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in lines {
        let split_line: Vec<&str> = line.split(" -> ").collect();
        let p1 = Point::from(split_line[0]);
        let p2 = Point::from(split_line[1]);

        if p1.x > max_x {
            max_x = p1.x;
        }

        if p2.x > max_x {
            max_x = p2.x
        }

        if p1.y > max_y {
            max_y = p1.y
        }

        if p2.y > max_y {
            max_y = p2.y
        }

        if p1.y > p2.y || p1.x > p2.x {
            blocks.push(Block { from: p2, to: p1 });
        } else {
            blocks.push(Block { from: p1, to: p2 });
        }
    }

    let mut field = vec![vec![0; max_y as usize + 1]; max_x as usize + 1];
    for block in blocks {
        if block.from.x != block.to.x && block.from.y != block.to.y {
            // ... *sigh*
            for i in 0..=(block.from.x-block.to.x).abs() {
                let x = block.from.x - (i * (block.from.x - block.to.x)/(block.from.x - block.to.x).abs());
                let y = block.from.y - (i * (block.from.y - block.to.y)/(block.from.y - block.to.y).abs());

                field[x as usize][y as usize] += 1;
            }
        } else {
            for x in block.from.x..=block.to.x {
                for y in block.from.y..=block.to.y {
                    field[x as usize][y as usize] += 1;
                }
            }
        }
    }

    let mut sum = 0;
    for x in 0..field.len() {
        for y in 0..field[x].len() {
            if field[x][y] > 1 {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}

impl From<&str> for Point {
    fn from(inp: &str) -> Point {
        let split: Vec<i32> = inp.split(',').map(| x | x.parse::<i32>().expect("Could not parse as number.")).collect();
        Point {
            x: split[0],
            y: split[1],
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

struct Block {
    from: Point,
    to: Point,
}