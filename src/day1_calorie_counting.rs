use itertools::Itertools;
use std::fs;

fn read() -> Vec<u32> {
    let file = fs::read_to_string("input/day1.txt").expect("File not found!");
    file.lines()
        .map(|line| line.parse::<u32>())
        .group_by(|line| line.is_ok())
        .into_iter()
        .filter_map(|(ok, values)| if ok { Some(values) } else { None })
        .map(|values| values.flatten().sum())
        .collect_vec()
}

pub fn calorie_counting() {
    let amounts = read();
    let maximum = amounts.iter().max().expect("No values given");
    println!("Maximum snack: {}", maximum);

    let max_three: u32 = amounts.iter().sorted_by(|a, b| b.cmp(a)).take(3).sum();
    println!("Maximum three: {}", max_three);
}
