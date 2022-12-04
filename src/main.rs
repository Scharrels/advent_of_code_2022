mod day1_calorie_counting;
mod day2_rock_paper_scissors;
mod day3_rucksack_reorganization;
mod day4_camp_cleanup;

use crate::day1_calorie_counting::calorie_counting;
use crate::day2_rock_paper_scissors::rock_paper_scissors;
use crate::day3_rucksack_reorganization::rucksack_reorganization;
use crate::day4_camp_cleanup::camp_cleanup;

fn main() {
    camp_cleanup();
}

#[allow(dead_code)]
fn previous() {
    calorie_counting();
    rock_paper_scissors();
    rucksack_reorganization();
}