use std::collections::HashSet;
use timer::profile;

pub fn run_day3(inputs: &String) {
    profile! {
        let day3_1 = day3_1(&inputs);
        println!("Day 3-1: {day3_1}");
    }

    profile! {
        let day3_2 = day3_2(&inputs);
        println!("Day 3-2: {day3_2}");
    }
}

fn day3_1(inputs: &String) -> usize {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert((x, y));

    for c in inputs.chars() {
        match c {
            '^' => y += 1,
            '<' => x -= 1,
            'v' => y -= 1,
            '>' => x += 1,
            _ => (),
        }
        visited.insert((x, y));
    }

    return visited.len();
}

fn day3_2(inputs: &String) -> usize {
    // Santa
    let mut x0: isize = 0;
    let mut y0: isize = 0;
    let mut s0: HashSet<(isize, isize)> = HashSet::new(); //Santa
    s0.insert((0, 0));
    // Robo-Santa
    let mut x1: isize = 0;
    let mut y1: isize = 0;
    let mut s1: HashSet<(isize, isize)> = HashSet::new(); //Robo-Santa
    s1.insert((0, 0));

    for (i, c) in inputs.chars().enumerate() {
        match c {
            '^' => {
                if i % 2 == 0 {
                    y0 += 1;
                } else {
                    y1 += 1;
                }
            }
            '<' => {
                if i % 2 == 0 {
                    x0 -= 1;
                } else {
                    x1 -= 1;
                }
            }
            'v' => {
                if i % 2 == 0 {
                    y0 -= 1;
                } else {
                    y1 -= 1;
                }
            }
            '>' => {
                if i % 2 == 0 {
                    x0 += 1;
                } else {
                    x1 += 1;
                }
            }
            _ => (),
        }

        if i % 2 == 0 {
            s0.insert((x0, y0));
        } else {
            s1.insert((x1, y1));
        }
    }

    return s0.union(&s1).collect::<Vec<_>>().len();
}
