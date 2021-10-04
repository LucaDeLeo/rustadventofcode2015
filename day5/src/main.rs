use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input");
    let mut n = 0;
    'outer: for line in input.lines() {
        let mut vowels = 0;
        let mut double: bool = false;
        let mut last: char = ' ';
        for c in line.chars() {
            match c {
                'a' => vowels += 1,
                'e' => vowels += 1,
                'i' => vowels += 1,
                'o' => vowels += 1,
                'u' => vowels += 1,
                _ => (),
            }
            if c == last {
                double = true
            }
            if (last, c) == ('a', 'b')
                || (last, c) == ('c', 'd')
                || (last, c) == ('p', 'q')
                || (last, c) == ('x', 'y')
            {
                continue 'outer;
            }
            last = c;
        }
        if vowels >= 3 && double {
            n += 1;
        }
    }
    println!("Part1: There are {} nice strings", n);
    let mut n2 = 0;
    for line in input.lines() {
        let mut lastlast = ' ';
        let mut last = ' ';
        let mut pair = false;
        let mut double = false;
        for (i, c) in line.chars().enumerate() {
            if c == lastlast {
                double = true;
            }
            if !pair && last != ' ' {
                let tmp = line.split_at(i);
                let mut st = last.to_string();
                st.push_str(&c.to_string());
                if tmp.0[..i - 1].contains(&st) || tmp.1[1..].contains(&st) {
                    pair = true;
                }
            }
            lastlast = last;
            last = c;
        }
        if pair && double {
            n2 += 1;
        }
    }
    println!("Part2: There are {} nice strings", n2);
}
