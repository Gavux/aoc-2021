use std::fs;

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let input = fs::read_to_string("input.txt").expect("Could not read file.");
    let lines: Vec<&str> = input.split('\n').collect();

    let mut boards: Vec<Vec<Vec<i32>>> = Vec::new();

    let mut board: Vec<Vec<i32>> = Vec::new();

    for i in 2..lines.len() {
        if lines[i].is_empty() {
            boards.push(board);
            board = Vec::new();
            continue;
        }

        let nums: Vec<&str> = lines[i].split_whitespace().collect();
        let mut row: Vec<i32> = Vec::new();
        for num in nums {
            row.push(num.parse::<i32>().expect("Could not parse as number"));
        }

        board.push(row);
    }

    let nums: Vec<i32> = lines[0].split(',').map(| x | x.parse::<i32>().expect("Could not parse as number")).collect();

    for num in nums {
        for i in 0..boards.len() {
            for x in 0..boards[i].len() {
                for y in 0..boards[i][x].len() {
                    if boards[i][x][y] == num {
                        boards[i][x][y] = -1;
                    
                        if check_for_win(&boards[i]) {
                            winning_board(&boards[i], num);
                            return;
                        }
                    }
                }
            }
        }
    }
}

fn part_b() {
    let input = fs::read_to_string("input.txt").expect("Could not read file.");
    let lines: Vec<&str> = input.split('\n').collect();

    let mut boards: Vec<Vec<Vec<i32>>> = Vec::new();

    let mut board: Vec<Vec<i32>> = Vec::new();

    for i in 2..lines.len() {
        if lines[i].is_empty() {
            boards.push(board);
            board = Vec::new();
            continue;
        }

        let nums: Vec<&str> = lines[i].split_whitespace().collect();
        let mut row: Vec<i32> = Vec::new();
        for num in nums {
            row.push(num.parse::<i32>().expect("Could not parse as number"));
        }

        board.push(row);
    }

    let nums: Vec<i32> = lines[0].split(',').map(| x | x.parse::<i32>().expect("Could not parse as number")).collect();

    let mut last_board = boards[0].clone();
    let mut last_num = 0;
    for num in nums {
        for i in 0..boards.len() {
            for x in 0..boards[i].len() {
                for y in 0..boards[i][x].len() {
                    if boards[i][x][y] == num {
                        boards[i][x][y] = -1;
                    
                        if check_for_win(&boards[i]) {
                            last_board = boards[i].clone();
                            last_num = num;
                        }
                    }
                }
            }
        }
    }

    winning_board(&last_board, last_num);
}

fn check_for_win(board: &Vec<Vec<i32>>) -> bool {
    let mut won = false;
    for x in 0..board.len() {
        let mut hor_sum = 0;
        let mut vert_sum = 0;

        for y in 0..board[x].len() {
            hor_sum += board[x][y];
            vert_sum += board[y][x];
        }

        if hor_sum == -(board[x].len() as i32) {
            if !won {
                won = true;
            } else {
                return false;
            }
        }
        
        if vert_sum == -(board[x].len() as i32) {
            if !won {
                won = true;
            } else {
                return false;
            }
        }
    }

    return won;
}

fn winning_board(board: &Vec<Vec<i32>>, num: i32) {    
    let mut sum = 0;

    for x in 0..board.len() {
        for y in 0..board[x].len() {
            if board[x][y] != -1 {
                sum += board[x][y];
            }
        }
    }

    println!("{}", sum * num);
}