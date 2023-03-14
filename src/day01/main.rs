use std::cmp::Ordering;

use nom::{
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

fn main() {
    let input = include_str!("./data.txt");
    let (_, elves) = parse_elves(input).unwrap();
    let be = find_max_calories(&elves).unwrap();
    println!("max calories: {:?}", be.total_calories());

    println!("max 3 calories: {:?}", find_max_3_calories(&elves).unwrap());
}

fn find_max_calories(elves: &[Elf]) -> Option<&Elf> {
    elves.iter().max_by(|a, b| a.cmp(b))
}

fn find_max_3_calories(elves: &[Elf]) -> Option<u32> {
    let mut c = elves.to_vec();
    c.sort_by(|a, b| b.cmp(a));

    let res = c[0..3].iter().map(|e| e.total_calories()).sum();
    Some(res)
}
#[derive(Debug, Clone)]
struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, calories) = separated_list1(newline, Elf::parse_calories)(s)?;
        Ok((s, Elf { calories }))
    }

    fn total_calories(&self) -> u32 {
        self.calories.iter().sum()
    }

    fn parse_calories(s: &str) -> IResult<&str, u32> {
        complete::u32(s)
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        self.total_calories().cmp(&other.total_calories())
    }
}

fn parse_elves(s: &str) -> IResult<&str, Vec<Elf>> {
    let doublenewline = tuple((newline, newline));
    separated_list1(doublenewline, Elf::parse)(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn top_elf() {
        let example = include_str!("./sample.txt");
        let (res, elves) = parse_elves(example).unwrap();
        assert_eq!(res, "");

        let result = find_max_calories(&elves).unwrap();
        assert_eq!(result.total_calories(), 24000);
    }

    #[test]
    fn top_3_elves() {
        let example = include_str!("./sample.txt");
        let (res, elves) = parse_elves(example).unwrap();
        assert_eq!(res, "");

        let result = find_max_3_calories(&elves).unwrap();
        assert_eq!(result, 45000);
    }
}
