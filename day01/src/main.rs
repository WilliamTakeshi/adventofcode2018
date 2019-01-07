use std::collections::HashSet;

fn main() {
    let res01 = result01("input.txt");
    println!("{}", res01);
    let res02 = result02("input.txt");
    println!("{}", res02);
}



fn result01(input: &str) -> i64 {
    let input = include_str!("input.txt");

    let mut count = 0;

    for line in input.lines() {
        let change: i64 = line.parse().unwrap();
        count += change;
    }
    count
}

fn result02(input: &str) -> i64 {
    let input = include_str!("input.txt");
    let mut count = 0;
    let mut v = HashSet::new();
    v.insert(0);
    loop {
        for line in input.lines() {
            let change: i64 = line.parse().unwrap();
            count += change;
            if v.contains(&count) {
                return count
            }
            v.insert(count);
        }
    }
}