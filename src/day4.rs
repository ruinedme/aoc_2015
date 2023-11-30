use md5::md5hash;
use timer::profile;

pub fn run_day4(inputs: &String) {
    profile! {
        let day4_1 = day4_1(&inputs);
        println!("Day 4-1: {day4_1}");
    }

    profile! {
        let day4_2 = day4_2(&inputs);
        println!("Day 4-2: {day4_2}");
    }
}

fn day4_1(inputs: &String) -> usize {
    let base = inputs.to_owned();
    for i in 0..u64::MAX {
        let t = format!("{}{}",base,i);
        let hash = md5hash(t.as_bytes());
        if hash[0] == 0 && hash[1] == 0 && hash[2] < 10 {
            return i as usize;
        }
    }

    println!("Didn't Find any valid hashes between 0 and {}", u64::MAX);
    return 0;
}

fn day4_2(inputs: &String) -> usize {
    let base = inputs.to_owned();
    for i in 0..u64::MAX {
        let t = format!("{}{}",base,i);
        let hash = md5hash(t.as_bytes());
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return i as usize;
        }
    }

    println!("Didn't Find any valid hashes between 0 and {}", u64::MAX);
    return 0;
}
