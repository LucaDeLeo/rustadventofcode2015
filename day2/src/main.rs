use std::cmp::min;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input");

    let mut fribbon: i32 = 0;
    let mut sum: i32 = 0;
    let mut nums: Vec<i32> = vec![0, 0, 0];
    let mut i: usize;
    for l in input.lines() {
        i = 0;
        for n in l.split('x') {
            nums[i] = n.parse::<i32>().unwrap();
            i += 1;
        }
        let (a, b, c): (i32, i32, i32) = (nums[0], nums[1], nums[2]);
        sum += 2 * (a * b + a * c + b * c) + min(min(a * b, a * c), b * c);
        fribbon += 2 * (a + b + c - nums.iter().max().unwrap()) + a * b * c;
    }
    println!(
        "Part1: The total square feet is : {}\nPart2: The total feet of ribbon is: {}",
        sum, fribbon
    );
}
