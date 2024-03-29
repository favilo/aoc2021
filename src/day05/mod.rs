use std::cmp::{max, min};

use anyhow::Result;
use itertools::zip;
use ndarray::Array2;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map, sequence::tuple, IResult,
};

use crate::{utils::parse_int, Runner};

pub type Point = (usize, usize);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    Straight,
    Diag,
}

pub struct Day;

impl Runner for Day {
    type Input = Array2<usize>;

    fn day() -> usize {
        5
    }

    fn get_input(input: &str) -> Result<Self::Input> {
        let mut grid = Array2::<usize>::zeros((1000, 1000));
        input
            .as_bytes()
            .split(|&c| c == '\n' as u8 || c == '\r' as u8)
            .filter(|b| b != b"")
            .map(parse_line)
            .map(Result::unwrap)
            .map(|t| t.1)
            .flatten()
            .for_each(|(t, p)| {
                match t {
                    Type::Straight => {
                        let n = grid.get_mut(p).unwrap();
                        *n += 1;
                    }
                    Type::Diag => {
                        let n = grid.get_mut(p).unwrap();
                        *n += 0x1_0000_0000;
                    }
                };
            });
        Ok(grid)
    }

    fn part1(input: &Self::Input) -> Result<usize> {
        Ok(input
            .into_iter()
            .filter(|&n| (n & 0xffff_ffff) > 1usize)
            .count())
    }

    fn part2(input: &Self::Input) -> Result<usize> {
        Ok(input
            .into_iter()
            .filter(|&n| {
                let hi = (n >> 32) & 0xffff_ffff;
                let lo = n & 0xffff_ffff;
                (lo + hi) > 1usize
            })
            .count())
    }
}

fn parse_line(input: &[u8]) -> IResult<&[u8], Vec<(Type, Point)>> {
    let number = |input| -> IResult<&[u8], usize> { map(digit1, |s: &[u8]| parse_int(s))(input) };
    let (input, (x1, _, y1, _, x2, _, y2)) = tuple((
        number,
        tag(b","),
        number,
        tag(b" -> "),
        number,
        tag(b","),
        number,
    ))(input)?;
    let vec: Vec<_> = if x1 == x2 {
        (min(y1, y2)..=max(y1, y2))
            .map(|y| (Type::Straight, (x1, y)))
            .collect()
    } else if y1 == y2 {
        (min(x1, x2)..=max(x1, x2))
            .map(|x| (Type::Straight, (x, y1)))
            .collect()
    } else {
        let xs: Vec<_> = if x1 > x2 {
            (x2..=x1).rev().collect()
        } else {
            (x1..=x2).collect()
        };
        let ys: Vec<_> = if y1 > y2 {
            (y2..=y1).rev().collect()
        } else {
            (y1..=y2).collect()
        };
        zip(xs, ys).map(|p| (Type::Diag, p)).collect()
    };
    Ok((input, vec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() -> Result<()> {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        let input = Day::get_input(input)?;
        println!("{:?}", input);
        assert_eq!(5, Day::part1(&input)?);
        assert_eq!(12, Day::part2(&input)?);
        Ok(())
    }
}
