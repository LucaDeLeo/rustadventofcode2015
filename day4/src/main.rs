fn main() {
    let secret_key = "ckczppom".to_string();
    let mut n = 0;
    let mut part1: Option<i32> = None;
    loop {
        let hash = md5::compute(secret_key.clone() + &n.to_string());
        let p = format!("{:x}", hash);
        if part1 == None && &p[0..5] == "00000" {
            part1 = Some(n);
        } else if &p[0..6] == "000000" {
            break;
        }
        n += 1;
    }
    println!("Part 1: the answer is: {}\nPart 2: the answer is: {}", part1.unwrap(), n);
}
