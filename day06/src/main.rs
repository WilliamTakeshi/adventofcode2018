use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");
    result01();
    result02();
}

fn result01() -> () {
    let input = include_str!("input.txt");
    let mut hm = HashMap::new();
    let mut start_points = Vec::new();
    let re = Regex::new(r"(\d+), (\d+)").unwrap();

    let mut x_min: i32 = 999999;
    let mut y_min: i32 = 999999;
    let mut x_max: i32 = 0;
    let mut y_max: i32 = 0;

    for cap in re.captures_iter(input) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        start_points.push((x, y));
        if x < x_min {
            x_min = x;
        }
        if y < y_min {
            y_min = y;
        }
        if x > x_max {
            x_max = x;
        }
        if y > y_max {
            y_max = y;
        }
    }
    for i in (x_min - 1)..=(x_max + 2) {
        for j in (y_min - 1)..=(y_max + 2) {
            let mut dist_min: i32 = 999999;
            for point in &start_points {
                let dist = manhatan_distance(*point, (i, j));
                if dist < dist_min {
                    hm.insert((i, j), point);
                    dist_min = dist;
                } else if dist == dist_min {
                    hm.insert((i, j), &(0, 0));
                }
            }
        }
    }
    let mut counter = HashMap::new();
    let mut edge_points = HashSet::new();

    for (_coodinate, closest_point) in &hm {
        if _coodinate.0 == x_min - 1 || _coodinate.0 == x_max + 1 {
            edge_points.insert(closest_point);
        } else if _coodinate.1 == y_min - 1 || _coodinate.1 == y_max + 1 {
            edge_points.insert(closest_point);
        }
        let count = counter.entry(closest_point).or_insert(0);
        *count += 1;
    }
    for edge_point in &edge_points {
        counter.remove_entry(edge_point);
    }
    println!("{:?}", counter);
    println!("{:?}", edge_points);
}

fn result02() -> () {
    let input = include_str!("input.txt");
    let mut hm = HashMap::new();
    let mut start_points = Vec::new();
    let re = Regex::new(r"(\d+), (\d+)").unwrap();

    let mut x_min: i32 = 999999;
    let mut y_min: i32 = 999999;
    let mut x_max: i32 = 0;
    let mut y_max: i32 = 0;

    for cap in re.captures_iter(input) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        start_points.push((x, y));
        if x < x_min {
            x_min = x;
        }
        if y < y_min {
            y_min = y;
        }
        if x > x_max {
            x_max = x;
        }
        if y > y_max {
            y_max = y;
        }
    }
    for i in (x_min - 1)..=(x_max + 2) {
        for j in (y_min - 1)..=(y_max + 2) {
            let mut dist_total: i32 = 0;
            for point in &start_points {
                let dist = manhatan_distance(*point, (i, j));
                dist_total += dist
            }
            if dist_total < 10000 {
                hm.insert((i, j), 1);
            }
        }
    }

    println!("{:?}", hm.len());
}

fn manhatan_distance(start: (i32, i32), finish: (i32, i32)) -> i32 {
    return (start.0 - finish.0).abs() + (start.1 - finish.1).abs();
}

#[test]
fn test_manhatan_distance() {
    assert_eq!(manhatan_distance((1, 1), (2, 2)), 2);
    assert_eq!(manhatan_distance((-1, -1), (2, 2)), 6);
}
