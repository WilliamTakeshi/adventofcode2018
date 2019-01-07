extern crate regex;
use std::collections::HashMap;
use regex::Regex;
#[macro_use] extern crate itertools;

use itertools::Itertools;
fn main() {
    result02();
}



// fn result01() -> () {
//     let input = include_str!("input.txt");
//     let re = Regex::new(r"\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
//     let mut hm = HashMap::new();
//     for cap in re.captures_iter(input) {
//         let x: u32 = cap[2].parse().unwrap();
//         let y: u32 = cap[3].parse().unwrap();
//         let x_change: u32 = cap[4].parse().unwrap();
//         let y_change: u32 = cap[5].parse().unwrap();
//         for i in x+1..x+x_change+1 {
//             for j in y+1..y+y_change+1 {
//                 let counter = hm.entry((i, j)).or_insert(0);
//                 *counter += 1;
//             }
//         }
//     }

//     println!("{}", hm.values().filter(|&v| v > &1).count());
// }


fn result02() -> () {
    let input = include_str!("input.txt");
    let re = Regex::new(r"\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let mut hm = HashMap::new();
    for cap in re.captures_iter(input) {
        let x: u32 = cap[2].parse().unwrap();
        let y: u32 = cap[3].parse().unwrap();
        let x_change: u32 = cap[4].parse().unwrap();
        let y_change: u32 = cap[5].parse().unwrap();
        for i in x+1..x+x_change+1 {
            for j in y+1..y+y_change+1 {
                let counter = hm.entry((i, j)).or_insert(0);
                *counter += 1;
            }
        }
    }

    println!("{:?}", hm);

    for cap2 in re.captures_iter(input) {
        let x: u32 = cap2[2].parse().unwrap();
        let y: u32 = cap2[3].parse().unwrap();
        let x_change: u32 = cap2[4].parse().unwrap();
        let y_change: u32 = cap2[5].parse().unwrap();
        if iproduct!((x+1..x+x_change+1), (y+1..y+y_change+1)).all(|(x,y)| hm.get(&(x,y)).unwrap() == &1) {
            println!("{}", &cap2[1]);
        }
    }
}