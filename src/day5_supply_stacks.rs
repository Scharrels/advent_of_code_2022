use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::fs;

#[derive(Display, FromStr, Debug)]
#[display("move {amount} from {from} to {to}")]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn read() -> (Vec<Vec<char>>, Vec<Instruction>) {
    let file = fs::read_to_string("input/day5.txt").expect("File not found!");
    let mut lines = file.lines();
    let mut stack_lines = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .chunks(4)
                .into_iter()
                .flat_map(|mut chars| chars.nth(1))
                .map(|char| if char == ' ' { None } else { Some(char) })
                .collect_vec()
        })
        .collect_vec();
    let number_of_stacks = stack_lines.pop().unwrap().len();
    let concatenated_stacks = stack_lines.iter().flatten().copied().collect_vec();
    let mut transposed_stacks = vec![None; concatenated_stacks.len()];
    transpose::transpose(
        &concatenated_stacks,
        &mut transposed_stacks,
        number_of_stacks,
        concatenated_stacks.len() / number_of_stacks,
    );
    let stacks = transposed_stacks
        .iter()
        .chunks(concatenated_stacks.len() / number_of_stacks)
        .into_iter()
        .map(|chars| {
            chars
                .flatten()
                .collect_vec()
                .iter()
                .rev()
                .copied()
                .copied()
                .collect_vec()
        })
        .collect_vec();
    let instructions = lines
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
        .unwrap();

    (stacks, instructions)
}

fn rearrange_9000(stacks: &[Vec<char>], instructions: &Vec<Instruction>) -> String {
    let mut private_stacks = stacks.to_owned();
    for instruction in instructions {
        let range = (private_stacks[instruction.from - 1].len() - instruction.amount)..;
        let blocks_to_move = private_stacks[instruction.from - 1]
            .drain(range)
            .rev()
            .collect_vec();
        private_stacks
            .get_mut(instruction.to - 1)
            .unwrap()
            .extend(blocks_to_move);
    }
    private_stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

fn rearrange_9001(stacks: &[Vec<char>], instructions: &Vec<Instruction>) -> String {
    let mut private_stacks = stacks.to_owned();
    for instruction in instructions {
        let range = (private_stacks[instruction.from - 1].len() - instruction.amount)..;
        let blocks_to_move = private_stacks[instruction.from - 1]
            .drain(range)
            .collect_vec();
        private_stacks
            .get_mut(instruction.to - 1)
            .unwrap()
            .extend(blocks_to_move);
    }
    private_stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

pub fn supply_stacks() {
    let (stacks, instructions) = read();
    println!(
        "CrateMover 9000: {}",
        rearrange_9000(&stacks, &instructions)
    );
    println!(
        "CrateMover 9001: {}",
        rearrange_9001(&stacks, &instructions)
    );
}
