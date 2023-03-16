use nom::{
    bytes::complete::tag,
    character::complete::{anychar, newline, u16 as parse_u16},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Board {
    pub tiles: Vec<Vec<TileElement>>,
}

#[derive(Debug, PartialEq)]
pub struct TileElement {
    pub val: char,
}

impl TileElement {
    pub fn new(val: char) -> Self {
        Self { val }
    }
}

impl fmt::Display for TileElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.val)
    }
}

impl Board {
    pub fn first_row(&self) -> String {
        let mut res = "".to_string();
        for tile in self.tiles.iter() {
            if let Some(el) = tile.last() {
                res = format!("{}{}", res, el.val)
            }
        }

        res
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let mut parse_rows = separated_list1(newline, parse_row);
        let (input, mut rows) = parse_rows(input)?;
        let (input, _) = newline(input)?;
        let (input, cnt) = parse_index(input)?;
        rows.reverse();

        let mut tiles: Vec<Vec<TileElement>> = vec![];
        for i in 0..cnt {
            let mut tile: Vec<TileElement> = vec![];
            for row in rows.iter() {
                if let Some(el) = &row[i] {
                    tile.push(TileElement::new(el.val));
                }
            }
            tiles.push(tile);
        }

        Ok((input, Self { tiles }))
    }

    pub fn width(&self) -> usize {
        self.tiles.len()
    }

    pub fn height(&self) -> usize {
        self.tiles.iter().map(|t| t.len()).max().unwrap_or(0)
    }

    pub fn at(&self, x: usize, y: usize) -> Option<TileElement> {
        if let Some(row) = self.tiles.get(x) {
            if let Some(el) = row.get(y) {
                return Some(TileElement::new(el.val));
            }
        }
        None
    }

    pub fn move_el(&mut self, from: usize, to: usize) {
        let tile = self.tiles[from].pop().unwrap();
        self.tiles[to].push(tile);
    }

    pub fn move_many_el(&mut self, count: usize, from: usize, to: usize) {
        let len = self.tiles[from].len();
        let from_range = len - count;
        let moved: Vec<_> = self.tiles[from].splice(from_range..len, vec![]).collect();
        self.tiles[to].extend(moved);
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Board")?;
        let w = self.width();
        let h = self.height();

        for j in 0..h {
            for i in 0..w {
                let el = self.at(i, h - j - 1);
                match el {
                    Some(v) => write!(f, "{} ", v)?,
                    None => write!(f, "    ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_row(input: &str) -> IResult<&str, Vec<Option<TileElement>>> {
    separated_list1(tag(" "), parse_element)(input)
}

fn parse_index(input: &str) -> IResult<&str, usize> {
    let (input, list) = separated_list1(tag(" "), tuple((tag(" "), parse_u16, tag(" "))))(input)?;
    Ok((input, list.len()))
}

fn parse_element(input: &str) -> IResult<&str, Option<TileElement>> {
    if let Ok((input, el)) = _parse_existing(input) {
        return Ok((input, Some(el)));
    }

    match _parse_empty(input) {
        Ok((input, _)) => Ok((input, None)),
        Err(e) => Err(e),
    }
}

fn _parse_existing(input: &str) -> IResult<&str, TileElement> {
    let (input, (_, val, _)) = tuple((tag("["), anychar, tag("]")))(input)?;
    let el = TileElement::new(val);
    Ok((input, el))
}

fn _parse_empty(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("   ")(input)?;
    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_element() {
        assert_eq!(
            parse_element("[a]"),
            Ok(("", Some(TileElement { val: 'a' })))
        );
        assert_eq!(
            parse_element("[B]"),
            Ok(("", Some(TileElement { val: 'B' })))
        );
    }

    #[test]
    fn test_parse_row() {
        let (_, res) = parse_row("[a]     [c]").unwrap();
        assert_eq!(
            res,
            vec![
                Some(TileElement::new('a')),
                None,
                Some(TileElement::new('c')),
            ]
        );
    }

    #[test]
    fn test_parse_board() {
        let input = include_str!("./sample.txt");
        let (_, board) = Board::parse(input).unwrap();
        assert_eq!(
            board,
            Board {
                tiles: vec![
                    vec![TileElement::new('Z'), TileElement::new('N'),],
                    vec![
                        TileElement::new('M'),
                        TileElement::new('C'),
                        TileElement::new('D'),
                    ],
                    vec![TileElement::new('P'),],
                ],
            }
        );
    }
}
