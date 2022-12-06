mod day1_calorie_counting;
mod day2_rock_paper_scissors;
mod day3_rucksack_reorganization;
mod day4_camp_cleanup;
mod day5_supply_stacks;
mod day6_tuning_trouble;

use crate::day1_calorie_counting::calorie_counting;
use crate::day2_rock_paper_scissors::rock_paper_scissors;
use crate::day3_rucksack_reorganization::rucksack_reorganization;
use crate::day4_camp_cleanup::camp_cleanup;
use crate::day5_supply_stacks::supply_stacks;
use crate::day6_tuning_trouble::tuning_trouble;

fn main() {
    tuning_trouble();
}

#[allow(dead_code)]
fn previous() {
    calorie_counting();
    rock_paper_scissors();
    rucksack_reorganization();
    camp_cleanup();
    supply_stacks();
}