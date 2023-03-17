use nom::branch::alt;
use nom::bytes::complete::take_while_m_n;
use nom::{character::complete::anychar, IResult};

fn main() {
    let input = include_str!("./data.txt");
    let res = parse_marker_or_consume_char(input);
    println!("res {}!", res.unwrap());

    let res = parse_message_marker_or_consume_char(input);
    println!("res message {}!", res.unwrap());
}

fn parse_marker_or_consume_char(input: &str) -> Option<usize> {
    parse_n_marker_or_consume_char(input, 4)
}

fn parse_message_marker_or_consume_char(input: &str) -> Option<usize> {
    parse_n_marker_or_consume_char(input, 14)
}

fn parse_n_marker_or_consume_char(input: &str, len: usize) -> Option<usize> {
    let mut inp = input;
    for i in 0..input.len() {
        let (res, c) = alt((parse_marker(len), consume_char))(inp).ok()?;
        if c.len() == len {
            return Some(i + len);
        }
        inp = res
    }

    None
}

fn is_digit(c: char) -> bool {
    c.is_alphanumeric()
}

fn parse_marker(len: usize) -> impl Fn(&str) -> IResult<&str, String> {
    move |input| {
        let (input, marker) = take_while_m_n(len, len, is_digit)(input)?;
        if check_if_duplicates(marker) {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
        Ok((input, marker.to_string()))
    }
}

fn check_if_duplicates(input: &str) -> bool {
    if input.len() <= 1 {
        return false;
    }

    let first = input.chars().next().unwrap();
    let substr = &input[1..];
    for c in substr.chars() {
        if c == first {
            return true;
        }
    }

    check_if_duplicates(&input[1..])
}

fn consume_char(input: &str) -> IResult<&str, String> {
    let (input, c) = anychar(input)?;
    Ok((input, c.to_string()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_marker_or_consume_char() {
        assert_eq!(
            parse_marker_or_consume_char("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            Some(5)
        );
        assert_eq!(
            parse_marker_or_consume_char("nppdvjthqldpwncqszvftbrmjlhg"),
            Some(6)
        );
        assert_eq!(
            parse_marker_or_consume_char("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            Some(10)
        );
        assert_eq!(
            parse_marker_or_consume_char("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            Some(11)
        );
    }

    #[test]
    fn test_parse_message_marker_or_consume_char() {
        assert_eq!(
            parse_message_marker_or_consume_char("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            Some(19)
        );
        assert_eq!(
            parse_message_marker_or_consume_char("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            Some(23)
        );
        assert_eq!(
            parse_message_marker_or_consume_char("nppdvjthqldpwncqszvftbrmjlhg"),
            Some(23)
        );
        assert_eq!(
            parse_message_marker_or_consume_char("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            Some(29)
        );
        assert_eq!(
            parse_message_marker_or_consume_char("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            Some(26)
        );
    }
}
