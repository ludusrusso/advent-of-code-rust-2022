use nom::bytes::complete::tag;
use nom::character::complete::{self, newline};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

fn main() {
    let input = include_str!("./data.txt");
    println!("Fully Overlapped: {}", get_fully_overlapped(input));
    println!("Partially Overlapped: {}", get_partial_overlapped(input));
}

#[derive(Debug, PartialEq)]
struct Range {
    start: u32,
    end: u32,
}

fn get_fully_overlapped(input: &str) -> u32 {
    let (_, ranges) = separated_list1(newline, Range::parse_pair)(input).unwrap();
    ranges
        .iter()
        .filter(|(first, second)| first.fully_overlaps(second))
        .count() as u32
}

fn get_partial_overlapped(input: &str) -> u32 {
    let (_, ranges) = separated_list1(newline, Range::parse_pair)(input).unwrap();
    ranges
        .iter()
        .filter(|(first, second)| first.partial_overlaps(second))
        .count() as u32
}

impl Range {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (start, end)) = separated_pair(complete::u32, tag("-"), complete::u32)(input)?;
        Ok((input, Self { start, end }))
    }

    fn parse_pair(input: &str) -> IResult<&str, (Self, Self)> {
        separated_pair(Self::parse, tag(","), Self::parse)(input)
    }

    fn fully_contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn fully_overlaps(&self, other: &Self) -> bool {
        self.fully_contains(other) || other.fully_contains(self)
    }

    fn partial_overlaps(&self, other: &Self) -> bool {
        !(self.end < other.start || other.end < self.start)
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_parse() {
        let input = "1-6";
        let (input, range) = Range::parse(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(range, Range { start: 1, end: 6 });
    }

    #[test]
    fn test_couple_parse() {
        let input = "1-6,4-7";
        let (input, (first, second)) = Range::parse_pair(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(first, Range { start: 1, end: 6 });
        assert_eq!(second, Range { start: 4, end: 7 });
    }

    #[test]
    fn test_overlaps() {
        assert!(!Range { start: 1, end: 6 }.fully_overlaps(&Range { start: 4, end: 7 }));
        assert!(Range { start: 1, end: 6 }.fully_overlaps(&Range { start: 2, end: 3 }));
        assert!(Range { start: 1, end: 6 }.fully_overlaps(&Range { start: 1, end: 9 }));
    }

    #[test]
    fn test_fully_overlapped() {
        let input = include_str!("./sample.txt");
        let res = get_fully_overlapped(input);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_partially_overlapped() {
        let input = include_str!("./sample.txt");
        let res = get_partial_overlapped(input);
        assert_eq!(res, 4);
    }
}
