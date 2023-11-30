use regex_lite::Regex;
use timer::profile;

pub fn run_day2(inputs: &String) {
    profile! {
        let day2_1 = day2_1(&inputs);
        println!("Day 2-1: {day2_1}");
    }

    profile! {
        let day2_2 = day2_2(&inputs);
        println!("Day 2-2: {day2_2}");
    }
}

fn day2_1(inputs: &String) -> usize {
    let re = Regex::new(r"^(?<l>\d+)x(?<w>\d+)x(?<h>\d+)$").unwrap();
    let mut total = 0;
    for line in inputs.lines() {
        let caps = re.captures(line).unwrap();
        let l: &usize = &caps["l"].parse().unwrap();
        let w: &usize = &caps["w"].parse().unwrap();
        let h: &usize = &caps["h"].parse().unwrap();
        let dim: [&usize; 3] = [&(l * w), &(w * h), &(h * l)];
        let smallest = *dim.iter().min().unwrap();

        total += (2 * dim[0]) + (2 * dim[1]) + (2 * dim[2]) + smallest;
    }
    return total;
}

fn day2_2(inputs: &String) -> usize {
    let re = Regex::new(r"^(?<l>\d+)x(?<w>\d+)x(?<h>\d+)$").unwrap();
    let mut total = 0;
    for line in inputs.lines() {
        let caps = re.captures(line).unwrap();
        let l: &usize = &caps["l"].parse().unwrap();
        let w: &usize = &caps["w"].parse().unwrap();
        let h: &usize = &caps["h"].parse().unwrap();
        let mut dim: [usize; 3] = [*l, *w, *h];
        dim.sort();

        let ribbon = (2 * dim[0] + 2 * dim[1]) + (l * w * h);
        total += ribbon;
    }
    return total;
}
