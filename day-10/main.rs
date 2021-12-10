use std::collections::HashMap;

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let input = include_str!("input.txt");
    let mut tokens: HashMap<char, char> = HashMap::new();
    tokens.insert('<', '>');
    tokens.insert('[', ']');
    tokens.insert('(', ')');
    tokens.insert('{', '}');

    let mut cost: HashMap<char, u32> = HashMap::new();
    cost.insert('>', 25137);
    cost.insert(']', 57);
    cost.insert(')', 3);
    cost.insert('}', 1197);

    let mut sum = 0;
    for line in input.split('\n').collect::<Vec<&str>>() {
        let mut open: Vec<char> = Vec::new();
        for token in line.chars() {
            if tokens.contains_key(&token) {
                open.push(token);
            } else {
                if open.len() == 0 || *tokens.get(&open.last().unwrap()).unwrap() != token {
                    sum += cost.get(&token).unwrap();
                    break;
                }
                open.remove(open.len() - 1);
            }
        }
    }

    println!("{}", sum);
}

fn part_b() {
    let input = include_str!("input.txt");
    let mut tokens: HashMap<char, char> = HashMap::new();
    tokens.insert('<', '>');
    tokens.insert('[', ']');
    tokens.insert('(', ')');
    tokens.insert('{', '}');

    let mut cost: HashMap<char, u64> = HashMap::new();
    cost.insert('>', 4);
    cost.insert(']', 2);
    cost.insert(')', 1);
    cost.insert('}', 3);

    let mut scores: Vec<u64> = Vec::new();
    for line in input.split('\n').collect::<Vec<&str>>() {
        let mut open: Vec<char> = Vec::new();
        let mut corrupted = false;

        for token in line.chars() {
            if tokens.contains_key(&token) {
                open.push(token);
            } else {
                 if *tokens.get(&open.last().unwrap()).unwrap() != token {
                    corrupted = true;
                    break;
                }
                open.remove(open.len() - 1);
            }
        }

        if !corrupted {
            open.reverse();
            let mut sum = 0;
            for closing in open {
                sum *= 5;
                sum += cost.get(tokens.get(&closing).unwrap()).unwrap();
            }

            scores.push(sum);
        }
    }

    scores.sort();

    println!("{}", scores[(scores.len() - 1) / 2]);
}