#![cfg(test)]

use std::{iter::Sum, str::FromStr};

trait Position: Sum<Direction> {
    fn depth(&self) -> u32;
    fn horizontal(&self) -> u32;
}

#[derive(Debug, Default)]
struct Position1 {
    depth: u32,
    horizontal: u32,
}

impl Position for Position1 {
    fn depth(&self) -> u32 {
        self.depth
    }

    fn horizontal(&self) -> u32 {
        self.horizontal
    }
}

impl Position for Position2 {
    fn depth(&self) -> u32 {
        self.depth
    }

    fn horizontal(&self) -> u32 {
        self.horizontal
    }
}

#[derive(Debug, Default)]
struct Position2 {
    depth: u32,
    horizontal: u32,
    aim: u32,
}

enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl Sum<Direction> for Position1 {
    fn sum<I: Iterator<Item = Direction>>(iter: I) -> Self {
        iter.fold(Position1::default(), |mut pos, movement| {
            match movement {
                Direction::Forward(amount) => pos.horizontal += amount,
                Direction::Up(amount) => pos.depth -= amount,
                Direction::Down(amount) => pos.depth += amount,
            }
            pos
        })
    }
}

impl Sum<Direction> for Position2 {
    fn sum<I: Iterator<Item = Direction>>(iter: I) -> Self {
        iter.fold(Position2::default(), |mut pos, movement| {
            match movement {
                Direction::Forward(amount) => {
                    pos.horizontal += amount;
                    pos.depth += amount * pos.aim;
                }
                Direction::Up(amount) => pos.aim -= amount,
                Direction::Down(amount) => pos.aim += amount,
            }
            pos
        })
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let direction = words
            .next()
            .ok_or_else(|| "missing direction".to_string())?;
        let amount = words.next().ok_or_else(|| "missing amount".to_string())?;
        let amount = amount.parse::<u32>().map_err(|err| err.to_string())?;

        match direction {
            "forward" => Ok(Direction::Forward(amount)),
            "up" => Ok(Direction::Up(amount)),
            "down" => Ok(Direction::Down(amount)),
            other => Err(format!("bad direction string: {}", other)),
        }
    }
}

fn calculate<T: Position>(data: &str) -> u32 {
    let position: T = data
        .lines()
        .map(|line| line.parse::<Direction>().unwrap())
        .sum();

    position.depth() * position.horizontal()
}

mod part_1 {
    use super::*;

    #[test]
    fn example() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        assert_eq!(calculate::<Position1>(input), 150);
    }

    #[test]
    fn solution() {
        let data = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day_02"));
        let answer = calculate::<Position1>(data);
        assert_eq!(answer, 1815044);
    }
}

mod part_2 {
    use super::*;

    #[test]
    fn example() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        assert_eq!(calculate::<Position2>(input), 900);
    }

    #[test]
    fn solution() {
        let data = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day_02"));
        let answer = calculate::<Position2>(data);
        assert_eq!(answer, 1739283308);
    }
}
