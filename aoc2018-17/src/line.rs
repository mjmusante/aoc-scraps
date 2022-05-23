use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum Line {
    Horiz(i64, (i64, i64)),
    Vert(i64, (i64, i64)),
}

impl Line {
    pub fn iter(&self) -> LineIter<'_> {
        LineIter::new(self)
    }

    pub fn parse(input: &str) -> Result<Self, String> {
        match Line::parse_str(input) {
            Ok((_, result)) => Ok(result),
            Err(_) => Err(format!("Unable to parse input: '{input:?}'")),
        }
    }

    fn parse_str(input: &str) -> IResult<&str, Self> {
        let rowcol = separated_pair(Line::alternative, char('='), Line::number_parser);
        let range = separated_pair(
            Line::alternative,
            char('='),
            separated_pair(Line::number_parser, tag(".."), Line::number_parser),
        );
        let r = separated_pair(rowcol, tag(", "), range);

        map(r, |((coord, rowcol), (_, (start, end)))| {
            if coord == "x" {
                Line::Vert(rowcol, (start, end))
            } else {
                Line::Horiz(rowcol, (start, end))
            }
        })(input)
    }

    fn alternative(input: &str) -> IResult<&str, &str> {
        alt((tag("x"), (tag("y"))))(input)
    }

    fn number_parser(input: &str) -> IResult<&str, i64> {
        map_res(digit1, i64::from_str)(input)
    }
}

pub struct LineIter<'a> {
    line: &'a Line,
    row: i64,
    col: i64,
}

impl<'a> LineIter<'a> {
    pub fn new(line: &'a Line) -> Self {
        match line {
            Line::Horiz(r, (c1, c2)) => Self {
                line,
                row: *r,
                col: *c1.min(c2),
            },
            Line::Vert(c, (r1, r2)) => Self {
                line,
                row: *r1.min(r2),
                col: *c,
            },
        }
    }
}

impl<'a> Iterator for LineIter<'a> {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        match self.line {
            Line::Horiz(_, w) => {
                if self.col > w.1 {
                    None
                } else {
                    self.col += 1;
                    Some((self.row, self.col - 1))
                }
            }
            Line::Vert(_, w) => {
                if self.row > w.1 {
                    None
                } else {
                    self.row += 1;
                    Some((self.row - 1, self.col))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal() {
        let hline = Line::Horiz(23, (10, 14));
        assert_eq!(
            vec![(23, 10), (23, 11), (23, 12), (23, 13), (23, 14)],
            hline.iter().collect::<Vec<(i64, i64)>>()
        );
    }

    #[test]
    fn test_vertial() {
        let vline = Line::Vert(3, (8, 12));
        assert_eq!(
            vec![(8, 3), (9, 3), (10, 3), (11, 3), (12, 3)],
            vline.iter().collect::<Vec<(i64, i64)>>()
        );
    }

    #[test]
    fn test_parsing_vertical() {
        let vline = Line::parse("x=495, y=2..7").unwrap();
        assert_eq!(vline, Line::Vert(495, (2, 7)));
    }

    #[test]
    fn test_parsing_horizontal() {
        let hline = Line::parse("y=13, x=498..504").unwrap();
        assert_eq!(hline, Line::Horiz(13, (498, 504)));
    }
}
