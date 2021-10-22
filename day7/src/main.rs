use std::fs;
#[derive(Debug)]
#[derive(PartialEq)]
enum Instruction {
    AND,
    OR,
    NOT,
    RSHIFT,
    LSHIFT,
    ASSIGN,
}
#[derive(Debug)]
struct Gate {
    pam1: String,
    ins: Instruction,
    pam2: String,
    dest: String,
}
fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input");
    let mut ins: Vec<Gate> = Vec::with_capacity(input.lines().count());
    let mut part2 = false;
    let mut a = "".to_string();

    'start: loop {
        for line in input.lines() {
            let mut tmp: Vec<String> = line
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            tmp.reverse();
            let g = Gate {
                pam1: if tmp.len() == 5 {
                    tmp[4].clone()
                } else {
                    "".to_string()
                },
                ins: if tmp.len() == 4 {
                    Instruction::NOT
                } else if tmp.len() == 5 {
                    // chooses the correct instruction based on the string
                    match tmp[3].as_ref() {
                        "AND" => Instruction::AND,
                        "OR" => Instruction::OR,
                        "NOT" => Instruction::NOT,
                        "RSHIFT" => Instruction::RSHIFT,
                        "LSHIFT" => Instruction::LSHIFT,
                        _ => Instruction::ASSIGN,
                    }
                } else {
                    Instruction::ASSIGN
                },
                pam2: tmp[2].clone(),
                dest: tmp[0].clone(),
            };
            ins.push(g);
        }

        if part2 {
            for g in ins.iter_mut() {
                if g.ins == Instruction::ASSIGN && g.dest == "b" {
                    g.pam2 = a;
                    break;
                }
            }
        }

        loop {
            let mut search = "".to_string();
            let mut replace = "".to_string();
            for (i, g) in ins.iter_mut().enumerate() {
                search = g.dest.clone();
                if g.pam1.parse::<u16>().is_ok() && g.pam2.parse::<u16>().is_ok() {
                    match g.ins {
                        Instruction::AND => {
                            g.dest = (g.pam1.parse::<u16>().unwrap()
                                & g.pam2.parse::<u16>().unwrap())
                            .to_string()
                        }
                        Instruction::OR => {
                            g.dest = (g.pam1.parse::<u16>().unwrap()
                                | g.pam2.parse::<u16>().unwrap())
                            .to_string()
                        }
                        Instruction::RSHIFT => {
                            g.dest = (g.pam1.parse::<u16>().unwrap()
                                >> g.pam2.parse::<u16>().unwrap())
                            .to_string()
                        }
                        Instruction::LSHIFT => {
                            g.dest = (g.pam1.parse::<u16>().unwrap()
                                << g.pam2.parse::<u16>().unwrap())
                            .to_string()
                        }
                        _ => continue,
                    }
                } else if g.pam2.parse::<u16>().is_ok() {
                    match g.ins {
                        Instruction::NOT => g.dest = (!g.pam2.parse::<u16>().unwrap()).to_string(),
                        Instruction::ASSIGN => g.dest = g.pam2.clone(),
                        _ => continue,
                    }
                } else {
                    continue;
                }
                replace = ins.swap_remove(i).dest;
                if search == "a" {
                    if !part2 {
                        println!("Part1: The signal ultimately provided to a is {}", replace);
                        a = replace;
                        part2 = true;
                        continue 'start;
                    } else {
                        println!("Part2: The signal ultimately provided to a is {}", replace);
                        break 'start;
                    }
                }
                break;
            }

            for g in &mut ins {
                if g.pam1 == search {
                    g.pam1 = replace.clone();
                }
                if g.pam2 == search {
                    g.pam2 = replace.clone();
                }
            }
        }
    }
}
