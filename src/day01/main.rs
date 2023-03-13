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
    print!("max calories: {:?}\n", be.total_calories());
}

fn find_max_calories(elves: &Vec<Elf>) -> Option<&Elf> {
    elves
        .iter()
        .max_by(|a, b| a.total_calories().cmp(&b.total_calories()))
}

fn parse_calory(s: &str) -> IResult<&str, u32> {
    complete::u32(s)
}

#[derive(Debug)]
struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    fn parse(s: &str) -> IResult<&str, Elf> {
        let (s, calories) = separated_list1(newline, parse_calory)(s)?;
        Ok((s, Elf { calories }))
    }

    fn total_calories(&self) -> u32 {
        self.calories.iter().sum()
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
    fn it_works() {
        let example = include_str!("./sample.txt");
        let (res, elves) = parse_elves(example).unwrap();
        assert_eq!(res, "");

        let result = find_max_calories(&elves).unwrap();
        assert_eq!(result.total_calories(), 24000);
    }
}
