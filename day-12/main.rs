use std::collections::HashMap;

fn main() {
    part_a_b();
}

fn part_a_b() {
    let input = include_str!("input.txt");
    let lines: Vec<&'static str> = input.split('\n').collect();
   
    let mut paths: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
    for &line in &lines {
        let split_line: Vec<&str> = line.split('-').collect();
        let mut vals = match paths.get(&split_line[0]) {
            Some(k) => k.clone(),
            _ => Vec::new(),
        };
        vals.push(&split_line[1]);
        paths.insert(&split_line[0], vals);

        let mut other_vals = match paths.get(&split_line[1]) {
            Some(k) => k.clone(),
            _ => Vec::new(),
        };
        other_vals.push(&split_line[0]);
        paths.insert(&split_line[1], other_vals);
    }


    println!("{:?}", path_a(&paths, vec!["start"]));
    println!("{:?}", path_b(&paths, vec!["start"], false));
}

fn path_a(paths: &HashMap<&'static str, Vec<&'static str>>, visited: Vec<&'static str>) -> u32 {
    if *visited.last().unwrap() == "end" {
        return 1;
    }

    paths.get(visited.last().unwrap()).unwrap().iter().map(| &x | {
        if (x.chars().all(char::is_lowercase) && visited.contains(&x)) || x == "start" {
            return 0;
        }

        let mut next = visited.clone();
        next.push(x);
        path_a(paths, next)
    }).sum()
}

fn path_b(paths: &HashMap<&'static str, Vec<&'static str>>, visited: Vec<&'static str>, visited_twice: bool) -> u32 {
    if *visited.last().unwrap() == "end" {
        return 1;
    }

    paths.get(visited.last().unwrap()).unwrap().iter().map(| &x | {
        if x == "start" {
            return 0;
        }

        let mut next_visited_twice = visited_twice;
        if x.chars().all(char::is_lowercase) && visited.contains(&x) {
            if visited_twice {
                return 0;
            }

            next_visited_twice = true;
        }

        let mut next = visited.clone();
        next.push(x);
        path_b(paths, next, next_visited_twice)
    }).sum()
}