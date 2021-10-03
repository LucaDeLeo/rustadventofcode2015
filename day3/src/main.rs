use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input");
    {
        let mut houses: HashMap<(i32, i32), i32> = HashMap::new();
        let mut pos: (i32, i32) = (0, 0);
        for c in input.chars() {
            match c {
                'v' => pos.0 += 1,
                '<' => pos.1 -= 1,
                '^' => pos.0 -= 1,
                '>' => pos.1 += 1,
                _ => (),
            }
            *houses.entry(pos).or_insert(0) += 1;
        }
        println!("Part 1: The number of houses is {}", houses.len());
    }
    let mut houses: HashMap<(i32, i32), i32> = HashMap::new();
    let mut pos_santa: (i32, i32) = (0, 0);
    let mut pos_robot: (i32, i32) = (0, 0);
    let mut even: bool = true;
    for c in input.chars() {
        let pos = if even { &mut pos_santa } else { &mut pos_robot };
        match c {
            'v' => pos.0 += 1,
            '<' => pos.1 -= 1,
            '^' => pos.0 -= 1,
            '>' => pos.1 += 1,
            _ => (),
        }
        even = !even;
        *houses.entry(*pos).or_insert(0) += 1;
    }
    println!("Part 2: The number of houses is {}", houses.len());
}
