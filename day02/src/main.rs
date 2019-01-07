use std::collections::HashMap;


fn main() {
    let re01 = result01();

    println!("{}", re01);
    let re02 = result02();

    println!("{:?}", re02);
}


fn result01() -> u64 {
    let input = include_str!("input.txt");

    let mut have2 = 0;
    let mut have3 = 0;

    for line in input.lines() {
        let mut letters = HashMap::new();
        for ch in line.chars() {
            let counter = letters.entry(ch).or_insert(0);
            *counter += 1;
        }
        if letters.values().any(|&v| v == 2) {
            have2 += 1;
        }
        if letters.values().any(|&v| v == 3) {
            have3 += 1;
        }
    }

    have2 * have3
}


fn result02() -> Option<(String, String)> {
    let input = include_str!("input.txt");

    for l1 in input.lines() {
        for l2 in input.lines().skip(1) {
            if l1.chars().zip(l2.chars()).filter(|(c1, c2)| c1 != c2).count() == 1 {
                return Some((l1.to_string(), l2.to_string()))
            } 
        }
    }
    None
}

// fn hm_chars(input: &str) -> HashMap