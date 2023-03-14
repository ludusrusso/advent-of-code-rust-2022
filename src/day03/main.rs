use std::error::Error;

use nom::branch::alt;
use nom::character::complete::{alphanumeric1, newline};
use nom::multi::{many1, separated_list1};
use nom::sequence::tuple;
use nom::IResult;

fn main() {
    let input = include_str!("./data.txt");
    let (_, score) = compute_score(input).unwrap();
    println!("score {}", score);

    let (_, badges_scores) = compute_3_elves_badge_scores(input).unwrap();
    println!("badges score {}", badges_scores);
}

fn compute_score(input: &str) -> IResult<&str, u32> {
    let (res, scores) = separated_list1(newline, parse_line_score)(input)?;
    let scores = scores.iter().sum();
    return Ok((res, scores));
}

fn compute_3_elves_badge_scores(input: &str) -> IResult<&str, u32> {
    let (res, scores) = separated_list1(newline, parse_3_elf_badge)(input)?;
    let scores = scores.iter().sum();
    return Ok((res, scores));
}

fn parse_line(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, line) = alphanumeric1(input)?;
    let len = line.len() / 2;
    let left = &line[0..len];
    let right = &line[len..];
    return Ok((input, (left, right)));
}

fn parse_3_lines(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let (input, (elf1, _)) = tuple((alphanumeric1, newline))(input)?;
    let (input, (elf2, _)) = tuple((alphanumeric1, newline))(input)?;
    let (input, elf3) = alphanumeric1(input)?;
    return Ok((input, (elf1, elf2, elf3)));
}

fn parse_3_elf_badge(input: &str) -> IResult<&str, u32> {
    let (input, (elf1, elf2, elf3)) = parse_3_lines(input)?;
    for c in elf1.chars() {
        if elf2.contains(c) && elf3.contains(c) {
            return Ok((input, char_to_u32(&c)));
        }
    }
    Ok((input, 0))
}

fn parse_line_score(input: &str) -> IResult<&str, u32> {
    let (input, (l, r)) = parse_line(input)?;
    for c in l.chars() {
        if r.contains(c) {
            return Ok((input, char_to_u32(&c)));
        }
    }
    Ok((input, 0))
}

fn char_to_u32(c: &char) -> u32 {
    if *c >= 'A' && *c <= 'Z' {
        return *c as u32 - 'A' as u32 + 27;
    }
    return *c as u32 - 'a' as u32 + 1;
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_score() {
        let input = include_str!("./sample.txt");
        let (_, score) = compute_score(input).unwrap();
        assert_eq!(score, 157);
    }

    #[test]
    fn test_compute_3_elves_bedge_score() {
        let input = include_str!("./sample.txt");
        let (_, score) = compute_3_elves_badge_scores(input).unwrap();
        assert_eq!(score, 70);
    }

    #[test]
    fn test_parse_line() {
        let input = "abcdef";
        let (res, (left, right)) = parse_line(input).unwrap();
        assert_eq!(res, "");
        assert_eq!(left, "abc");
        assert_eq!(right, "def");
    }

    #[test]
    fn test_parse_duplicated_item() {
        let input = "abcaef";
        let (res, value) = parse_line_score(input).unwrap();
        assert_eq!(res, "");
        assert_eq!(value, 1);
    }

    #[test]
    fn test_parse_duplicated_item_z() {
        let input = "zbcazf";
        let (res, value) = parse_line_score(input).unwrap();
        assert_eq!(res, "");
        assert_eq!(value, 26);
    }

    #[test]
    fn test_parse_duplicated_item_a_uppercase() {
        let input = "AbcazA";
        let (res, value) = parse_line_score(input).unwrap();
        assert_eq!(res, "");
        assert_eq!(value, 27);
    }
}
