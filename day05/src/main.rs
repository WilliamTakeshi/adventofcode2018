use std::collections::{HashSet, HashMap};

fn main() {
    result02();
    println!("Hello, world!");
}

fn result01() -> () {
    let input = include_str!("input.txt");
    // let chars = input.chars().collect::<Vec<_>>();
    // for i in 0..chars.len(){
    //     println!("{:?}", chars[i]);
    // }
    let result = react_polymerer(input.to_string());
    println!("{}, {}", result, result.len());
}

fn result02() -> () {
    let mut input = include_str!("input.txt");
    // let chars = input.chars().collect::<Vec<_>>();
    // for i in 0..chars.len(){
    //     println!("{:?}", chars[i]);
    // }
    let result = react_polymerer(input.to_string());
    
    let mut new_results = HashMap::new();
    let mut letters = HashSet::new();
    for letter in result.to_lowercase().chars() {
        letters.insert(letter);
    }

    for letter in &letters {
        let new_input = result.replace(letter.clone(), "").replace(&letter.to_uppercase().to_string().clone(), "");
        let result_new = react_polymerer(new_input);
        new_results.insert(letter, result_new.len());
    }

    println!("{:?}", new_results);
    let mut count_vec: Vec<(&&char, &&usize)> = new_results.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("{:?}", count_vec);
        
}


fn react_polymerer(input: String) -> String {
    // let chars = input.chars().collect::<Vec<_>>();
    // for i in 0..chars.len(){
    //     println!("{:?}", chars[i]);
    // }
    let mut input = input;
    let mut aaa = String::new();
    loop {
        let result = input
            .chars()
            .zip(input.chars().skip(1))
            .filter(|&(a, b)| can_delete(a, b))
            .map(|(a,b)| format!("{}{}", a, b))
            .take(1)
            .collect::<Vec<_>>();

        if result.len() == 0 {
            break;
        }
        aaa = result.into_iter()
            .map(|v| input.replace(&v.clone(), ""))
            .collect();

        input = aaa;
    }
    input.to_string()
}

fn can_delete(a: char, b: char) -> bool {
    return a.to_lowercase().to_string() == b.to_lowercase().to_string()
        && a.is_lowercase() == b.is_uppercase();
}

#[test]
fn test_can_delete() {
    assert!(can_delete('a', 'A'));
    assert!(can_delete('B', 'b'));
    assert!(!can_delete('a', 'a'));
    assert!(!can_delete('a', 'b'));
    assert!(!can_delete('B', 'A'));
}
