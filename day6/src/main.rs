use std::fs;
enum Instruction {
    TurnOff,
    TurnOn,
    Toggle,
}
fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input");
    let mut grid = [[(false, 0u32); 1000]; 1000];

    for line in input.lines() {
        let parts: Vec<&str> = line.split("through").collect();
        let from = parts[0].split(",").collect::<Vec<&str>>();
        let to = parts[1].split(",").collect::<Vec<&str>>();

        let a = from[0]
            .matches(char::is_numeric)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let c = from[1]
            .matches(char::is_numeric)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let b = to[0]
            .matches(char::is_numeric)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let d = to[1]
            .matches(char::is_numeric)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let ins: Instruction;
        if line.starts_with("turn off") {
            ins = Instruction::TurnOff;
        } else if line.starts_with("turn on") {
            ins = Instruction::TurnOn;
        } else {
            ins = Instruction::Toggle;
        }

        for i in a..=b {
            for j in c..=d {
                match ins {
                    Instruction::TurnOn => {
                        grid[i][j].0 = true;
                        grid[i][j].1 += 1;
                    }
                    Instruction::TurnOff => {
                        grid[i][j].0 = false;
                        grid[i][j].1 = grid[i][j].1.saturating_sub(1);
                    }
                    Instruction::Toggle => {
                        grid[i][j].0 = !grid[i][j].0;
                        grid[i][j].1 += 2;
                    }
                }
            }
        }
    }
    let mut n = 0;
    let mut k: u32 = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if grid[i][j].0 {
                n += 1;
            }
            k += grid[i][j].1;
        }
    }
    println!(
        "Part1: There are {} lights lit.\nPart2: The total brightness is {}",
        n, k
    );
}
