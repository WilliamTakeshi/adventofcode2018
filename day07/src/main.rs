use regex::Regex;
use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    result01();
}

fn result01() -> () {
    let input = include_str!("input.txt");

    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
    let mut pre_requisites = HashMap::new();

    for cap in re.captures_iter(input) {
        let pre = cap[1].to_string();
        let pos = cap[2].to_string();
        pre_requisites.entry(pre.clone()).or_insert(Vec::new());
        let pre_req = pre_requisites.entry(pos).or_insert(Vec::new());
        pre_req.push(pre);
    }

    let mut initial_values = pre_requisites
        .clone()
        .into_iter()
        .filter(|(_key, value)| value.len() == 0)
        .map(|(key, _value)| key)
        .collect::<Vec<_>>();
    initial_values.sort();
    let initial_value = initial_values[0].to_string();
    println!("{:?}", initial_value);
    pre_requisites.remove(&initial_value);
    let mut finished_values = vec![initial_value];
    while &pre_requisites.len() > &0 {
        let mut next_steps = pre_requisites
            .iter()
            .filter(|(_key, value)| value.iter().all(|v| finished_values.contains(v)))
            .map(|(key, _value)| key)
            .collect::<Vec<_>>();
        next_steps.sort();
        let next_step = next_steps[0].to_string();
        pre_requisites.remove(&next_step);
        finished_values.push(next_step);
    }
    let result: String = finished_values.into_iter().collect();
    println!("{:?}", &result);
}
