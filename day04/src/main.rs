use std::collections::HashMap;
extern crate regex;

use regex::Regex;
fn main() {
    result01();
    println!("Hello, world!");
}



fn result01() -> () {
    let input = include_str!("input.txt");

    let mut sleep_hm = HashMap::new();
    let mut sleep_421 = HashMap::new();
    let mut vect = input.lines().collect::<Vec<_>>();
    vect.sort();
    let re_guard_begins = Regex::new(r"^\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}\] Guard #(\d+) begins shift$").unwrap();
    let re_falls_asleep = Regex::new(r"^\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})\] falls asleep$").unwrap();
    let re_wakes_up = Regex::new(r"^\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})\] wakes up$").unwrap();
    let mut guard = "";
    let mut sleep_time: u32 = 0;    
    let mut wake_time: u32 = 0;  
    for line in vect.iter() {
        if re_guard_begins.is_match(line) {
            let cap = re_guard_begins.captures(line).unwrap();
            guard = cap.get(1).unwrap().as_str();
        } else if re_falls_asleep.is_match(line) {
            let cap = re_falls_asleep.captures(line).unwrap();
            sleep_time = cap.get(1).unwrap().as_str().parse().unwrap();
        } else if re_wakes_up.is_match(line) {
            let cap = re_wakes_up.captures(line).unwrap();
            wake_time = cap.get(1).unwrap().as_str().parse().unwrap();
            let time = sleep_hm.entry(guard).or_insert(0);
            *time += wake_time - sleep_time;
            for min in sleep_time..wake_time {
                let time421 = sleep_421.entry((guard,min)).or_insert(0);
                *time421 += 1;
            }
        }

    }
    let mut count_vec: Vec<(&(&str, u32), &u32)> = sleep_421.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("{:?}", count_vec);
}


