use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    result01(468, 718430);
}

fn result01(players: u32, last_marble: u32) -> () {
    let mut marble_mania = vec![0];
    let mut next_marble = 1;
    let mut score = HashMap::new();
    let mut current_marble_position = 0;
    let mut current_player = 0;
    loop {
        current_player = (current_player + 1) % players;
        if next_marble % 23 == 0 {
            let remove_marble_index = (current_marble_position + marble_mania.len() - 7) % marble_mania.len();
            let removed_marble = marble_mania.remove(remove_marble_index);
            let player_score = score.entry(current_player).or_insert(0);
            *player_score += next_marble + removed_marble;
            current_marble_position = remove_marble_index;
        } else {
            current_marble_position = 1 + (current_marble_position + 1) % marble_mania.len();
            marble_mania.insert(current_marble_position, next_marble);
        }
        next_marble += 1;
        if next_marble > last_marble {
            break;
        }
        // if next_marble % 1000 == 0 {
        //     println!("{:?}", next_marble);
        // }
    }
    println!("{:?}", score);
    let mut score_vec: Vec<(_,_)> = score.iter().collect();
    score_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("{}, {}", score_vec[0].0, score_vec[0].1);
}
