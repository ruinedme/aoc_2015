use timer::profile;

pub fn run_day6(inputs: &String) {
    profile! {
        let day6_1 = day6_1(&inputs);
        println!("Day 6-1: {day6_1}");
    }

    profile! {
        let day6_2 = day6_2(&inputs);
        println!("Day 6-2: {day6_2}");
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    NONE = 0,
    ON = 1,
    OFF = 2,
    TOGGLE = 3,
}

fn parse_instructions(inputs: &Vec<&str>) -> Vec<(Instruction, (usize, usize), (usize, usize))> {
    let mut instructions: Vec<(Instruction, (usize, usize), (usize, usize))> = Vec::new();
    inputs.iter().for_each(|&x| {
        let tokens: Vec<&str> = x.split(' ').collect();
        let mut inst = (Instruction::NONE, (0, 0), (0, 0));
        if tokens[0] == "toggle" {
            inst.0 = Instruction::TOGGLE;
            let range1: Vec<&str> = tokens[1].split(',').collect();
            let range2: Vec<&str> = tokens[3].split(',').collect();
            inst.1 = (range1[0].parse().unwrap(), range1[1].parse().unwrap());
            inst.2 = (range2[0].parse().unwrap(), range2[1].parse().unwrap());
        } else if tokens[0] == "turn" {
            if tokens[1] == "on" {
                inst.0 = Instruction::ON;
            } else {
                inst.0 = Instruction::OFF;
            }
            let range1: Vec<&str> = tokens[2].split(',').collect();
            let range2: Vec<&str> = tokens[4].split(',').collect();
            inst.1 = (range1[0].parse().unwrap(), range1[1].parse().unwrap());
            inst.2 = (range2[0].parse().unwrap(), range2[1].parse().unwrap());
        }
        instructions.push(inst);
    });

    return instructions;
}

fn process_instruction(instruction: (Instruction,(usize,usize), (usize,usize)), lights: &mut Vec<usize>, is_bool: bool) {
    let grid_width: usize = 1000;
    for y in instruction.1 .1..=instruction.2 .1 {
        for x in instruction.1 .0..=instruction.2 .0 {
            let i = grid_width * y + x;
            match instruction.0 {
                Instruction::OFF => {
                    if is_bool {
                        lights[i] = 0;
                    } else  if lights[i] > 0 {
                        lights[i] -= 1;
                    }
                },
                Instruction::ON => {
                    lights[i] += 1;
                },
                Instruction::TOGGLE => {
                    if is_bool {
                        if lights[i] == 0 {
                            lights[i] = 1;
                        } else {
                            lights[i] = 0;
                        }
                    } else {
                        lights[i] += 2;
                    }
                },
                Instruction::NONE => (),
            }
        }
    }
}

fn day6_1(inputs: &String) -> usize {
    let lines: Vec<&str> = inputs.lines().collect();
    let instructions = parse_instructions(&lines);
    let mut lights: Vec<usize> = vec![0; 1000000];

    instructions
        .iter()
        .for_each(|&instruction| {
            process_instruction(instruction, &mut lights, true);
        });

    return lights.iter().filter(|&x| *x > 0).count();
}

fn day6_2(inputs: &String) -> usize {
    let lines: Vec<&str> = inputs.lines().collect();
    let instructions = parse_instructions(&lines);
    let mut lights: Vec<usize> = vec![0; 1000000];

    instructions
        .iter()
        .for_each(|&instruction| {
            process_instruction(instruction, &mut lights, false);
        });

    return lights.iter().sum();
}
