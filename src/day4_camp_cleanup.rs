use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct ElfPairParseProblem; // It's an error, but elves adore alliteration

struct ElfPair {
    first_elf: RangeInclusive<u32>,
    second_elf: RangeInclusive<u32>,
}

impl ElfPair {
    fn get_range(input: &str) -> Result<RangeInclusive<u32>, ElfPairParseProblem> {
        let (start_str, end_str) = input.split_once('-').ok_or(ElfPairParseProblem)?;
        let start = start_str.parse::<u32>().map_err(|_| ElfPairParseProblem)?;
        let end = end_str.parse::<u32>().map_err(|_| ElfPairParseProblem)?;
        Ok(start..=end)
    }

    fn full_overlap(&self) -> bool {
        ((self.first_elf.start() <= self.second_elf.start())
            && self.first_elf.end() >= self.second_elf.end())
            || ((self.second_elf.start() <= self.first_elf.start())
                && self.second_elf.end() >= self.first_elf.end())
    }

    fn partial_overlap(&self) -> bool {
        ((self.first_elf.start() <= self.second_elf.start())
            && self.first_elf.end() >= self.second_elf.start())
            || ((self.second_elf.start() <= self.first_elf.start())
                && self.second_elf.end() >= self.first_elf.start())
    }
}

impl FromStr for ElfPair {
    type Err = ElfPairParseProblem;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (first_range, second_range) = input.split_once(',').ok_or(ElfPairParseProblem)?;
        let first_elf = ElfPair::get_range(first_range)?;
        let second_elf = ElfPair::get_range(second_range)?;
        Ok(ElfPair {
            first_elf,
            second_elf,
        })
    }
}

fn read() -> Vec<ElfPair> {
    let file = fs::read_to_string("input/day4.txt").expect("File not found!");
    file.lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .unwrap()
}

pub fn camp_cleanup() {
    let elf_pairs = read();
    let fully_overlapping_pairs = elf_pairs
        .iter()
        .filter(|elf_pair| elf_pair.full_overlap())
        .count();
    println!("Fully overlapping pairs: {}", fully_overlapping_pairs);

    let partially_overlapping_pairs = elf_pairs
        .iter()
        .filter(|elf_pair| elf_pair.partial_overlap())
        .count();
    println!(
        "Partially overlapping pairs: {}",
        partially_overlapping_pairs
    );
}
