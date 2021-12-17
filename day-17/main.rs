fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let input = include_str!("input.txt");
    let bounds = input.split("target area: ").collect::<Vec<_>>()[1]
        .split(", ")
        .collect::<Vec<_>>();
    let t_x = bounds[0].split("=").collect::<Vec<_>>()[1].split("..").map(| x | x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let t_y = bounds[1].split("=").collect::<Vec<_>>()[1].split("..").map(| x | x.parse::<i32>().unwrap()).collect::<Vec<_>>();

    let mut max_y = 0;
    for x in 0..=t_x[1] {
        for y in 0..=t_y[0].abs() {
            let mut v = (x, y);
            let mut c = (0, 0);
            let mut c_max_y = 0;
            while c.0 < t_x[1] && c.1 > t_y[0] {
                c.0 += v.0;
                c.1 += v.1;

                if c.0 >= t_x[0] && c.0 <= t_x[1] && c.1 >= t_y[0] && c.1 <= t_y[1] && c_max_y > max_y {
                    max_y = c_max_y;
                    break;
                }

                if c.1 > c_max_y {
                    c_max_y = c.1;
                }

                if v.0 > 0 {
                    v.0 -= 1;
                }

                v.1 -= 1;
            }
        }
    }

    println!("{}", max_y);
}

fn part_b() {
    let input = include_str!("input.txt");
    let bounds = input.split("target area: ").collect::<Vec<_>>()[1]
        .split(", ")
        .collect::<Vec<_>>();
    let t_x = bounds[0].split("=").collect::<Vec<_>>()[1].split("..").map(| x | x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let t_y = bounds[1].split("=").collect::<Vec<_>>()[1].split("..").map(| x | x.parse::<i32>().unwrap()).collect::<Vec<_>>();

    let mut hit_count = 0;
    for x in 0..=t_x[1] {
        for y in t_y[0]..=t_y[0].abs() {
            let mut v = (x, y);
            let mut c = (0, 0);
            while c.0 < t_x[1] && c.1 > t_y[0] {
                c.0 += v.0;
                c.1 += v.1;

                if c.0 >= t_x[0] && c.0 <= t_x[1] && c.1 >= t_y[0] && c.1 <= t_y[1] {
                    hit_count += 1;
                    break;
                }

                if v.0 > 0 {
                    v.0 -= 1;
                }

                v.1 -= 1;
            }
        }
    }

    println!("{}", hit_count);
}