fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let input = include_str!("input.txt");

    let lines: Vec<&str> = input.split('\n').collect();
    let mut field: Vec<Vec<u32>> = Vec::new();
    for &line in &lines {
        let mut row: Vec<u32> = Vec::new();
        for c in line.chars() {
            let digit = c.to_digit(10).unwrap();
            row.push(digit);
        }
        field.push(row);
    }

    let mut flashes = 0;
    let mut flashed: Vec<(u32, u32)> = Vec::new();
    for _ in 0..100 {
        for y in 0..field.len() {
            for x in 0..field[y].len() {
                field[y][x] += 1;
            }
        }

        for y in 0..field.len() {
            for x in 0..field[y].len() {
                if field[y][x] > 9 {
                    flash(&mut field, x, y, &mut flashes, &mut flashed);
                    flashes += 1;
                }
            }
        }

        flashed = Vec::new();
    }



    println!("{}", flashes);
}

fn part_b() {
    let input = include_str!("input.txt");

    let lines: Vec<&str> = input.split('\n').collect();
    let mut field: Vec<Vec<u32>> = Vec::new();
    for &line in &lines {
        let mut row: Vec<u32> = Vec::new();
        for c in line.chars() {
            let digit = c.to_digit(10).unwrap();
            row.push(digit);
        }
        field.push(row);
    }

    let mut flashed: Vec<(u32, u32)> = Vec::new();
    let mut first_all_flashes = 0;
    for i in 0..1000 {
        let mut flashes = 0;

        for y in 0..field.len() {
            for x in 0..field[y].len() {
                field[y][x] += 1;
            }
        }

        for y in 0..field.len() {
            for x in 0..field[y].len() {
                if field[y][x] > 9 {
                    flash(&mut field, x, y, &mut flashes, &mut flashed);
                    flashes += 1;
                }
            }
        }

        if flashes == (field.len() * field[0].len()) as u32 && first_all_flashes == 0 {
            first_all_flashes = i + 1;
        }

        flashed = Vec::new();
    }

    println!("{}", first_all_flashes);
}

fn increase(field: &mut Vec<Vec<u32>>, x: usize, y: usize, flashes: &mut u32, flashed: &mut Vec<(u32, u32)>) {
    if flashed.contains(&(x as u32, y as u32)) {
        return;
    }
    field[y][x] += 1;
    if field[y][x] > 9 {
        flash(field, x, y, flashes, flashed);
        *flashes += 1;
    }
}

fn flash(field: &mut Vec<Vec<u32>>, x: usize, y: usize, flashes: &mut u32, flashed: &mut Vec<(u32, u32)>) {
    if flashed.contains(&(x as u32, y as u32)) {
        return;
    }
    field[y][x] = 0;
    flashed.push((x as u32, y as u32));
    
    for i in -1 as i32..=1 {
        for j in -1 as i32..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            let next_y = y as i32 + i;
            let next_x = x as i32 + j;
            if next_y < 0 || next_x < 0 || next_y as usize >= field.len() || next_x as usize >= field[next_y as usize].len() {
                continue;
            }
            increase(field, next_x as usize, next_y as usize, flashes, flashed);
        }
    }
}