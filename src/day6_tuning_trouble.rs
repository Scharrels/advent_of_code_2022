use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

pub fn tuning_trouble() {
    let file = fs::read_to_string("input/day6.txt").expect("File not found!");
    let chars = file.chars().collect_vec();

    let marker_position = chars
        .windows(4)
        .take_while(|marker| HashSet::<&char>::from_iter(marker.iter()).len() != 4)
        .count();
    println!("marker after {} characters", marker_position + 4);

    let message_position = chars
        .windows(14)
        .take_while(|message| HashSet::<&char>::from_iter(message.iter()).len() != 14)
        .count();
    println!("marker after {} characters", message_position + 14);
}
