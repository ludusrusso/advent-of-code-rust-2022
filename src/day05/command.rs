use nom::{
    bytes::complete::tag,
    character::complete::{newline, u16 as parse_u16},
    multi::separated_list1,
    IResult,
};

use super::board::Board;

#[derive(Debug, PartialEq)]
pub struct Command {
    size: usize,
    from: usize,
    to: usize,
}

impl Command {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("move ")(input)?;
        let (input, size) = parse_u16(input)?;
        let (input, _) = tag(" from ")(input)?;
        let (input, from) = parse_u16(input)?;
        let (input, _) = tag(" to ")(input)?;
        let (input, to) = parse_u16(input)?;
        Ok((
            input,
            Self {
                size: size as usize,
                from: from as usize - 1,
                to: to as usize - 1,
            },
        ))
    }

    pub fn execute(&self, board: &mut Board) {
        for _ in 0..self.size {
            board.move_el(self.from, self.to);
        }
    }

    pub fn execute_v2(&self, board: &mut Board) {
        board.move_many_el(self.size, self.from, self.to)
    }

    pub fn parse_many(input: &str) -> IResult<&str, Vec<Self>> {
        separated_list1(newline, Self::parse)(input)
    }
}

mod tests {
    #[test]
    fn test_parse_command() {
        assert_eq!(
            Command::parse("move 5 from 1 to 3"),
            Ok((
                "",
                Command {
                    size: 5,
                    from: 0,
                    to: 2
                }
            ))
        );
    }

    #[test]
    fn test_command() {
        use super::*;

        let mut board = Board {
            tiles: vec![vec![TileElement::new('Z'), TileElement::new('N')], vec![]],
        };

        let command = Command {
            size: 1,
            from: 0,
            to: 1,
        };

        command.execute(&mut board);

        let expected = Board {
            tiles: vec![vec![TileElement::new('Z')], vec![TileElement::new('N')]],
        };

        assert_eq!(board, expected)
    }
}
