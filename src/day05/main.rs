use nom::bytes::complete::tag;
use nom::sequence::separated_pair;
use nom::IResult;

mod board;
mod command;

use board::Board;
use command::Command;

fn main() {
    let input = include_str!("./data.txt");
    let (_, (mut board, commands)) = parse_board_and_commands(input).unwrap();
    for c in commands {
        c.execute(&mut board);
    }

    println!("Board Row: {:?}", board.first_row());

    let input = include_str!("./data.txt");
    let (_, (mut board, commands)) = parse_board_and_commands(input).unwrap();
    for c in commands {
        c.execute_v2(&mut board);
    }

    println!("Board Row V2: {:?}", board.first_row());
}

fn parse_board_and_commands(input: &str) -> IResult<&str, (Board, Vec<Command>)> {
    separated_pair(Board::parse, tag("\n\n"), Command::parse_many)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = include_str!("./sample.txt");
        let (_, (mut board, commands)) = parse_board_and_commands(input).unwrap();
        for c in commands {
            c.execute(&mut board);
        }

        assert_eq!("CMZ", board.first_row());
    }

    #[test]
    fn test_sample_v2() {
        let input = include_str!("./sample.txt");
        let (_, (mut board, commands)) = parse_board_and_commands(input).unwrap();
        for c in commands {
            c.execute_v2(&mut board);
        }

        assert_eq!("MCD", board.first_row());
    }
}
