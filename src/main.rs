mod day1_calorie_counting;
mod day2_rock_paper_scissors;
mod day3_rucksack_reorganization;

use crate::day1_calorie_counting::calorie_counting;
use crate::day2_rock_paper_scissors::rock_paper_scissors;
use crate::day3_rucksack_reorganization::rucksack_reorganization;

fn main() {
    rucksack_reorganization();
}

#[allow(dead_code)]
fn previous() {
    calorie_counting();
    rock_paper_scissors();
}