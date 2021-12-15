use std::collections::BinaryHeap;

fn main() {
  part_a();
  part_b();
}

fn part_a() {
  let input = include_str!("input.txt");
  let field: Vec<Vec<i32>> = input
    .split('\n')
    .map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as i32).collect())
    .collect();

  println!("{}", shortest_path(&field));
}

fn part_b() {
  let input = include_str!("input.txt");
  let field: Vec<Vec<i32>> = input
    .split('\n')
    .map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as i32).collect())
    .collect();

  let big_field = (0..(5 * field.len()))
    .map(|x| {
      (0..(5 * field[0].len()))
        .map(|y| {
          let cost = field[x % field.len()][y % field[0].len()]
            + (x / field.len()) as i32
            + (y / field[0].len()) as i32;
          if cost < 10 {
            cost
          } else {
            cost - 9
          }
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

    println!("{}", shortest_path(&big_field));
}

fn shortest_path(field: &Vec<Vec<i32>>) -> i32 {
  let mut distances = vec![vec![i32::MAX; field[0].len()]; field.len()];
  let mut heap = BinaryHeap::new();
  heap.push((0, 0, 0));

  while let Some((cost, x, y)) = heap.pop() {
    if (x, y) == (field.len() - 1, field[0].len() - 1) {
      return -cost;
    }
    if -cost > distances[x][y] {
      continue;
    }

    for (x1, y1) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
      let next_cost = match field.get(x1).and_then(|row| row.get(y1)) {
        Some(c) => -cost + c,
        _ => continue,
      };

      if next_cost < distances[x1][y1] {
        distances[x1][y1] = next_cost;
        heap.push((-next_cost, x1, y1));
      }
    }
  }
  unreachable!()
}
