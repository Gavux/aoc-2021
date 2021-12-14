use std::collections::HashMap;

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let input = include_str!("input.txt");
    let split: Vec<&str> = input.split("\n\n").collect();
    let mut template: Vec<char> = split[0].chars().collect();
    let raw_rules: Vec<&str> = split[1].split("\n").collect();
    let mut rules: HashMap<String, char> = HashMap::new();
    for &rule in &raw_rules {
        let split_r: Vec<&str> = rule.split(" -> ").collect();
        rules.insert(String::from(split_r[0]), split_r[1].chars().collect::<Vec<char>>()[0]);
    }

    for _ in 0..10 {
        let mut to_insert: HashMap<usize, char> = HashMap::new();
        for i in 0..template.len() {
            if i == template.len() - 1 {
                break;
            }

            let combination = format!("{}{}", template[i], template[i + 1]);
            let insert = match rules.get(&combination) {
                Some(k) => k,
                _ => continue,
            };

            to_insert.insert(i + 1 + to_insert.len(), *insert);
        }

        let mut keys: Vec<_> = to_insert.keys().collect();
        keys.sort();
        for &key in &keys {
            template.insert(*key, *to_insert.get(key).unwrap());
        }
    }

    let mut counts: HashMap<char, u32> = HashMap::new();
    for &c in &template {
        let count = match counts.get(&c) {
            Some(k) => *k,
            _ => 0,
        };

        counts.insert(c, count + 1);
    }

    let max = counts.iter().max_by_key(| x | x.1).unwrap().1;
    let min = counts.iter().min_by_key(| x | x.1).unwrap().1;
    println!("{}", max - min);
}

fn part_b() {
    let input = include_str!("input.txt");
    let split: Vec<&str> = input.split("\n\n").collect();
    let template: Vec<char> = split[0].chars().collect();
    let raw_rules: Vec<&str> = split[1].split("\n").collect();
    let mut rules: HashMap<String, char> = HashMap::new();
    let mut chars: HashMap<String, u64> = HashMap::new();
    let mut counts: HashMap<char, u64> = HashMap::new();

    for &rule in &raw_rules {
        let split_r: Vec<&str> = rule.split(" -> ").collect();
        rules.insert(String::from(split_r[0]), split_r[1].chars().collect::<Vec<char>>()[0]);
        chars.insert(String::from(split_r[0]), 0);
    }

    for i in 0..template.len() {
        if i != template.len() - 1 {
            let combination = format!("{}{}", template[i], template[i + 1]);
            let count = *chars.get(&combination).unwrap();
            chars.insert(combination, count + 1);
        }
        
        let count = match counts.get(&template[i]) {
            Some(k) => *k,
            _ => 0,
        };

        counts.insert(template[i], count + 1);
    }

    for _ in 0..40 {
        for (key, value) in chars.clone().iter() {
            if *value == 0 {
                continue;
            }

            let rule = rules.get(key).unwrap();
            let c: Vec<char> = key.chars().collect();
            let first = format!("{}{}", c[0], rule);
            let second = format!("{}{}", rule, c[1]);

            let count = match counts.get(rule) {
                Some(k) => *k,
                _ => 0,
            };
    
            counts.insert(*rule, count + value);

            chars.insert(key.clone(), chars.get(key).unwrap() - value);
            chars.insert(first.clone(), chars.get(&first).unwrap() + value);
            chars.insert(second.clone(), chars.get(&second).unwrap() + value);
        }
    }

    let max = counts.iter().max_by_key(| x | x.1).unwrap().1;
    let min = counts.iter().min_by_key(| x | x.1).unwrap().1;
    println!("{}", max - min);
}