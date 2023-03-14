use nom::{
    bytes::streaming::tag,
    character::complete::{anychar, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("./data.txt");
    let (_, games) = parse_games(input).unwrap();
    let total_score = games.iter().map(|(a, b)| b.game_score(a)).sum::<i32>();
    println!("total score: {}", total_score);

    let (_, games) = parse_games_v2(input).unwrap();
    let total_score = games.iter().map(|(a, b)| b.game_score(a)).sum::<i32>();
    println!("total score: {}", total_score);
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum PlayChoice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum GameOutcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl GameOutcome {
    fn score(&self) -> i32 {
        *self as i32
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, c) = anychar(input)?;
        match c {
            'X' => Ok((input, GameOutcome::Lose)),
            'Y' => Ok((input, GameOutcome::Draw)),
            'Z' => Ok((input, GameOutcome::Win)),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::OneOf,
            ))),
        }
    }
}

fn parse_games(input: &str) -> IResult<&str, Vec<(PlayChoice, PlayChoice)>> {
    separated_list1(newline, PlayChoice::parse_pair)(input)
}

fn parse_games_v2(input: &str) -> IResult<&str, Vec<(PlayChoice, PlayChoice)>> {
    separated_list1(newline, PlayChoice::parse_pair_v2)(input)
}

impl PlayChoice {
    fn game_score(&self, other: &Self) -> i32 {
        self.game(other).score() + self.score()
    }

    fn parse_pair(input: &str) -> IResult<&str, (Self, Self)> {
        separated_pair(Self::parse, tag(" "), Self::parse_second)(input)
    }

    fn parse_pair_v2(input: &str) -> IResult<&str, (Self, Self)> {
        let (input, a) = Self::parse(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, outcome) = GameOutcome::parse(input)?;
        let b = a.other_to_outcome(outcome);

        Ok((input, (a, b)))
    }

    pub fn score(&self) -> i32 {
        *self as i32
    }

    fn game(&self, other: &Self) -> GameOutcome {
        let res = (*self as i32) - (*other as i32);
        match (res + 3) % 3 {
            0 => GameOutcome::Draw,
            1 => GameOutcome::Win,
            2 => GameOutcome::Lose,
            _ => panic!("invalid game outcome"),
        }
    }

    fn other_to_outcome(&self, outcome: GameOutcome) -> Self {
        match outcome {
            GameOutcome::Draw => *self,
            GameOutcome::Win => match self {
                PlayChoice::Rock => PlayChoice::Paper,
                PlayChoice::Paper => PlayChoice::Scissors,
                PlayChoice::Scissors => PlayChoice::Rock,
            },
            GameOutcome::Lose => match self {
                PlayChoice::Rock => PlayChoice::Scissors,
                PlayChoice::Paper => PlayChoice::Rock,
                PlayChoice::Scissors => PlayChoice::Paper,
            },
        }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, c) = anychar(input)?;
        match c {
            'A' => Ok((input, PlayChoice::Rock)),
            'B' => Ok((input, PlayChoice::Paper)),
            'C' => Ok((input, PlayChoice::Scissors)),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::OneOf,
            ))),
        }
    }

    fn parse_second(input: &str) -> IResult<&str, Self> {
        let (input, c) = anychar(input)?;
        match c {
            'X' => Ok((input, PlayChoice::Rock)),
            'Y' => Ok((input, PlayChoice::Paper)),
            'Z' => Ok((input, PlayChoice::Scissors)),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::OneOf,
            ))),
        }
    }
}

// implement test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_game() {
        assert_eq!(PlayChoice::Rock.game(&PlayChoice::Paper), GameOutcome::Lose);
        assert_eq!(PlayChoice::Rock.game(&PlayChoice::Rock), GameOutcome::Draw);
        assert_eq!(
            PlayChoice::Rock.game(&PlayChoice::Scissors),
            GameOutcome::Win
        );
        assert_eq!(
            PlayChoice::Scissors.game(&PlayChoice::Scissors),
            GameOutcome::Draw
        );
        assert_eq!(
            PlayChoice::Scissors.game(&PlayChoice::Rock),
            GameOutcome::Lose
        );
    }

    #[test]
    fn test_parse() {
        let input = include_str!("./sample.txt");
        let (res, games) = parse_games(input).unwrap();
        assert_eq!(res, "");
        assert_eq!(games.len(), 3);
        assert_eq!(games[0], (PlayChoice::Rock, PlayChoice::Paper));

        let total_score = games.iter().map(|(a, b)| b.game_score(a)).sum::<i32>();
        assert_eq!(total_score, 15);
    }

    #[test]
    fn test_parse_v2() {
        let input = include_str!("./sample.txt");
        let (res, games) = parse_games_v2(input).unwrap();
        assert_eq!(res, "");
        assert_eq!(games.len(), 3);
        assert_eq!(games[0], (PlayChoice::Rock, PlayChoice::Rock));

        let total_score = games.iter().map(|(a, b)| b.game_score(a)).sum::<i32>();
        assert_eq!(total_score, 12);
    }
}
