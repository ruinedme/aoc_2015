use timer::profile;

pub fn run_day25(inputs: &String) {
    profile! {
        let day25_1 = day25_1(&inputs);
        println!("Day 25-1: {day25_1}");
    }

    profile! {
        let day25_2 = day25_2(&inputs);
        println!("Day 25-2: {day25_2}");
    }
}

fn day25_1(inputs: &String) -> usize {
    let mut code: usize = 20151125;
    let m1: usize = 252533;
    let m2: usize = 33554393;

    let lines: Vec<&str> = inputs.lines().collect();
    let target_row: usize = lines[0].parse().unwrap();
    let target_col: usize = lines[1].parse().unwrap();

    let mut current_row: usize = 1;
    let mut highest_row: usize = 1;
    let mut current_col: usize = 1;
    // println!("{}, {} = {}", current_row, current_col, code);
    while current_row < target_row || current_col < target_col {
        let next = (code * m1) % m2;
        code = next;
        if current_row == 1 {
            highest_row += 1;
            current_row = highest_row;
            current_col = 1;
        } else {
            current_row -= 1;
            current_col += 1;
        }
        // println!("{}, {} = {}", current_row, current_col, code);
    }
    return code;
}

fn day25_2(inputs: &String) -> usize {
    return 0;
}