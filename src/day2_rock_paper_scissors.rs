use std::fs;
use std::str::FromStr;

static WIN_LOOKUP: &[u32] = &[
//  X  Y  Z
    1, 2, 0, // A (rock)
    0, 1, 2, // B (paper)
    2, 0, 1, // C (scissors)
];

#[derive(Debug, Clone)]
struct RoundParseError;

#[derive(Debug)]
struct Round {
    opponent_choice: u32,
    other_input: u32,
}

impl Round {
    fn points(&self, own_choice: u32) -> u32 {
        WIN_LOOKUP[(self.opponent_choice * 3 +  own_choice) as usize] * 3 + own_choice + 1
    }

    fn correct_points(&self) -> u32 {
        let own_choice = WIN_LOOKUP[((2 - self.opponent_choice) * 3 + self.other_input) as usize]; // reverse the "perspective" of the lookup table
        self.points(own_choice)
    }
}

impl FromStr for Round {
    type Err = RoundParseError;

    fn from_str(input: &str) -> Result<Round, Self::Err> {
        let mut chars = input.chars();
        let opponent_choice = chars.next().map(|char| char as u32 - 'A' as u32).ok_or(RoundParseError)?;
        let other_input = chars.nth(1).map(|char| char as u32 - 'X' as u32).ok_or(RoundParseError)?;
        Ok(Round { opponent_choice, other_input })
    }
}

fn read() -> Vec<Round> {
    let file = fs::read_to_string("input/day2.txt").expect("File not found!");
    file.lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>().unwrap()
}

pub fn rock_paper_scissors() {
    let rounds = read();

    let first_score = rounds.iter().map(|round| round.points(round.other_input)).sum::<u32>();
    println!("Total score: {}", first_score);

    let correct_score = rounds.iter().map(|round| round.correct_points()).sum::<u32>();
    println!("Correct score: {}", correct_score);
}
