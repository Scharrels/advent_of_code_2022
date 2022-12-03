use std::collections::HashSet;
use std::fs;

fn item_priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'A' as u32 + 27,
        'A'..='Z' => item as u32 - 'a' as u32 + 1,
        _ => 0,
    }
}

fn mismatched_item(contents: &String) -> u32 {
    let (compartment_1, compartment_2) = contents.split_at(contents.len() / 2);
    let sorted_1: HashSet<char> = compartment_1.chars().collect();
    let sorted_2: HashSet<char> = compartment_2.chars().collect();
    let common_element = *sorted_1
        .intersection(&sorted_2)
        .next()
        .expect("No common element found");
    item_priority(common_element)
}

fn find_badge(rucksacks: &[String]) -> u32 {
    let common_elements = rucksacks
        .iter()
        .map(|rucksack| rucksack.chars().collect::<HashSet<char>>())
        .reduce(|acc, item_set| acc.intersection(&item_set).copied().collect())
        .expect("No rucksacks found");
    let badge = *common_elements
        .iter()
        .next()
        .expect("No common element found");
    item_priority(badge)
}

fn read() -> Vec<String> {
    let file = fs::read_to_string("input/day3.txt").expect("File not found!");
    file.lines().map(|str| str.into()).collect()
}

pub fn rucksack_reorganization() {
    let rucksacks = read();
    let item_score: u32 = rucksacks.iter().map(mismatched_item).sum();
    println!("Priority (items): {}", item_score);

    let badge_score: u32 = rucksacks.chunks(3).map(find_badge).sum();
    println!("Priority (badges): {}", badge_score);
}
