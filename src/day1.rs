use timer::profile;

pub fn run_day1(inputs: &String) {
    profile! {
        let day1_1 = day1_1(&inputs);
        println!("Day 1-1: {day1_1}");
    }

    profile! {
        let day1_2 = day1_2(&inputs);
        println!("Day 1-2: {day1_2}");
    }
}

fn day1_1(inputs: &String) -> isize {
    let mut floor: isize = 0;
    for c in inputs.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    return floor;
}

fn day1_2(inputs: &String) -> usize {
    let mut floor: isize = 0;
    for (i, c) in inputs.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => {
                floor -= 1;
                if floor < 0 {
                    return i + 1;
                }
            }
            _ => (),
        }
    }
    return 0;
}
