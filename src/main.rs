mod day1_calorie_counting;
mod day2_rock_paper_scissors;

use crate::day1_calorie_counting::calorie_counting;
use crate::day2_rock_paper_scissors::rock_paper_scissors;

fn main() {
    rock_paper_scissors();
}

#[allow(dead_code)]
fn previous() {
    calorie_counting();
}