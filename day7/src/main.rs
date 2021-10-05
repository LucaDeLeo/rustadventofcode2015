use std::fs;
#[derive(Debug)]
enum Instruction {
    AND,
    OR,
    NOT,
    RSHIFT,
    LSHIFT,
    ASSIGN,
}
#[derive(Debug)]
struct Gate<'a> {
    pam1: &'a str,
    ins: Instruction,
    pam2: &'a str,
    dest: &'a str,
}
fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input");
    let mut ins: Vec<Gate> = Vec::new();

    for line in input.lines() {
        let mut tmp: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
        tmp.reverse();
        let g = Gate {
            pam1: if tmp.len() == 5 { tmp[4] } else { &" " },
            ins: if tmp.len() == 4 {
                Instruction::NOT
            } else if tmp.len() == 5 {
                match tmp[3] {
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
            pam2: tmp[2],
            dest: tmp[0],
        };
        ins.push(g);
    }

    loop {
        let mut restr = " ".to_string();
        let mut search: &str = " ";
        let mut res: u16 = 0;
        let mut index = 0;
        for (i, g) in &mut ins.iter().enumerate() {
            if g.pam1.parse::<u16>().is_ok() && g.pam2.parse::<u16>().is_ok() {
                match g.ins {
                    Instruction::AND => {
                        res = g.pam1.parse::<u16>().unwrap() & g.pam2.parse::<u16>().unwrap()
                    }
                    Instruction::OR => {
                        res = g.pam1.parse::<u16>().unwrap() | g.pam2.parse::<u16>().unwrap()
                    }
                    Instruction::RSHIFT => {
                        res = g.pam1.parse::<u16>().unwrap() >> g.pam2.parse::<u16>().unwrap()
                    }
                    Instruction::LSHIFT => {
                        res = g.pam1.parse::<u16>().unwrap() & g.pam2.parse::<u16>().unwrap()
                    }
                    _ => (),
                }
            } else if g.pam2.parse::<u16>().is_ok() {
                match g.ins {
                    Instruction::NOT => res = !g.pam2.parse::<u16>().unwrap(),
                    Instruction::ASSIGN => res = g.pam2.parse::<u16>().unwrap(),
                    _ => (),
                }
            } else {
                continue;
            }
            index = i;
            break;
        }
        if index == ins.len() {
            break;
        }
        restr = res.to_string();
        for g in &mut ins {
            if g.pam1 == search {
                g.pam1 = &restr[..];
            }
        }
    }
}
