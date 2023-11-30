use std::collections::HashMap;

use timer::profile;

pub fn run_day5(inputs: &String) {
    profile! {
        let day5_1 = day5_1(&inputs);
        println!("Day 5-1: {day5_1}");
    }

    profile! {
        let day5_2 = day5_2(&inputs);
        println!("Day 5-2: {day5_2}");
    }
}

fn day5_1(inputs: &String) -> usize {
    let mut nice = 0;

    for line in inputs.lines() {
        let mut vowels = 0;
        let mut has_double = false;
        let mut has_bad = false;
        let chars: Vec<char> = line.chars().collect();
        for (i, c) in chars.iter().enumerate() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    vowels += 1;
                    if i < chars.len() - 1 {
                        if c == &'a' && chars[i + 1] == 'b' {
                            has_bad = true;
                        }
                        if &chars[i + 1] == c {
                            has_double = true;
                        }
                    }
                }
                'c' | 'p' | 'x' => {
                    if i < chars.len() - 1 {
                        if (c == &'c' && chars[i + 1] == 'd')
                            || (c == &'p' && chars[i + 1] == 'q')
                            || (c == &'x' && chars[i + 1] == 'y')
                        {
                            has_bad = true;
                            break;
                        }
                        if &chars[i + 1] == c {
                            has_double = true;
                        }
                    }
                }
                _ => {
                    if i < chars.len() - 1 && !has_double {
                        if &chars[i + 1] == c {
                            has_double = true;
                        }
                    }
                }
            }
        }
        if vowels >= 3 && has_double && !has_bad {
            nice += 1;
        }
    }
    return nice;
}

fn day5_2(inputs: &String) -> usize {
    let mut nice = 0;

    for line in inputs.lines() {
        let mut pairs: HashMap<&str, usize> = HashMap::new();
        let mut has_repeat = false;
        let mut is_overlap = false;

        // get all pairs
        for i in 0..line.len() - 1 {
            let t = &line[i..i + 2];
            pairs
                .entry(t)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
            if i < line.len() - 2 {
                // has any repeating character
                if &line[i..i + 1] == &line[i + 2..i + 3] {
                    has_repeat = true;
                }
                // has overlapping repeating character
                if &line[i..i + 1] == &line[i + 2..i + 3] && &line[i..i + 1] == &line[i + 1..i + 2]
                {
                    is_overlap = true;
                }
            }
        }
        let dupe_pairs: HashMap<&str, usize> = pairs
            .iter()
            .filter_map(|x| if *x.1 > 1 { Some((*x.0, *x.1)) } else { None })
            .collect();

        let max = pairs.iter().max_by(|&x, &y| x.1.cmp(&y.1)).unwrap();

        // we have overlapping and at least 2 pairs
        if (is_overlap && dupe_pairs.len() >= 2) 
            || (has_repeat && !is_overlap && *max.1 >= 2)
            || (has_repeat && *max.1 >= 3 && is_overlap)
        // we have a sequence of characters that repeats 4 or more times
        {
            nice += 1;
        }
    }
    return nice;
}
