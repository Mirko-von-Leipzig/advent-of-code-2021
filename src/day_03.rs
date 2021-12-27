#![cfg(test)]

use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bitvec(Vec<bool>);

impl From<Bitvec> for u32 {
    fn from(bits: Bitvec) -> Self {
        bits.0.into_iter().fold(0, |mut acc, bit| {
            acc <<= 1;
            if bit {
                acc += 1;
            }
            acc
        })
    }
}

impl FromStr for Bitvec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits = s
            .bytes()
            .map(|byte| match byte {
                b'0' => Ok(false),
                b'1' => Ok(true),
                other => Err(format!("Invalid bit: {}", other as char)),
            })
            .collect::<Result<Vec<_>, Self::Err>>()?;

        Ok(Bitvec(bits))
    }
}

fn oxygen_rate<'a, T>(data: T) -> u32
where
    T: Iterator<Item = &'a Bitvec>,
{
    let mut data = data.collect::<HashSet<_>>();
    let mut idx = 0;

    while data.len() > 1 {
        let count = sum_column(data.iter().cloned(), idx).signum();
        data = data
            .into_iter()
            .filter(|bits| matches!((bits.0[idx], count), (true, 1) | (true, 0) | (false, -1)))
            .collect();
        idx += 1;
    }

    let result = data.into_iter().next().unwrap().clone();

    u32::from(result)
}

fn co2_scrubber_rate<'a, T>(data: T) -> u32
where
    T: Iterator<Item = &'a Bitvec>,
{
    let mut data = data.collect::<HashSet<_>>();
    let mut idx = 0;

    while data.len() > 1 {
        let count = sum_column(data.iter().cloned(), idx).signum();
        data = data
            .into_iter()
            .filter(|bits| matches!((bits.0[idx], count), (true, -1) | (false, 0) | (false, 1)))
            .collect();
        idx += 1;
    }

    let result = data.into_iter().next().unwrap().clone();

    u32::from(result)
}

fn sum_column<'a, T>(data: T, column: usize) -> i32
where
    T: Iterator<Item = &'a Bitvec>,
{
    data.fold(0, |mut acc, bits| {
        acc += match bits.0[column] {
            true => 1,
            false => -1,
        };
        acc
    })
}

fn power_consumption<'a, T>(data: T) -> u32
where
    T: Iterator<Item = &'a Bitvec> + Clone,
{
    let bits = data
        .clone()
        .peekable()
        .peek()
        .map(|bitvec| bitvec.0.len())
        .unwrap_or_default();

    let counts = (0..bits)
        .into_iter()
        .map(|idx| sum_column(data.clone(), idx))
        .collect::<Vec<_>>();

    let gamma = counts.iter().fold(0, |mut acc, val| {
        acc <<= 1;
        if val > &0 {
            acc += 1;
        }
        acc
    });

    let epsilon = (1 << counts.len()) - 1 - gamma;

    gamma * epsilon
}

fn example_str() -> &'static str {
    "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"
}

fn parse_input_str(input: &str) -> Vec<Bitvec> {
    input
        .lines()
        .map(|line| line.parse::<Bitvec>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

mod part_1 {
    use super::*;

    #[test]
    fn example() {
        let input = example_str();
        let input = parse_input_str(input);
        assert_eq!(power_consumption(input.iter()), 198);
    }

    #[test]
    fn solution() {
        let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day_03"));
        let input = parse_input_str(input);
        assert_eq!(power_consumption(input.iter()), 2250414);
    }
}

mod part_2 {
    use super::*;

    #[test]
    fn oxygen_rating() {
        let input = example_str();
        let input = parse_input_str(input);
        assert_eq!(oxygen_rate(input.iter()), 23);
    }

    #[test]
    fn co2_scrubber_rating() {
        let input = example_str();
        let input = parse_input_str(input);
        assert_eq!(co2_scrubber_rate(input.iter()), 10);
    }

    #[test]
    fn solution() {
        let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day_03"));
        let input = parse_input_str(input);

        let o2 = oxygen_rate(input.iter());
        let co2 = co2_scrubber_rate(input.iter());

        assert_eq!(o2 * co2, 6085575);
    }
}
