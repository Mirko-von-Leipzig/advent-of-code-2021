#![cfg(test)]

use crate::iter::WindowIterator;

fn running_increases<const WINDOW_LEN: usize>(data: &str) -> usize {
    data.lines()
        .map(|line| u32::from_str_radix(line, 10).unwrap())
        .window::<WINDOW_LEN>()
        .map(|measurements| measurements.iter().sum::<u32>())
        .window::<2>()
        .filter(|pairs| pairs[1] > pairs[0])
        .count()
}

mod part_1 {
    use super::*;

    #[test]
    fn example() {
        let test_data = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
        assert_eq!(running_increases::<1>(test_data), 7);
    }

    #[test]
    fn answer() {
        let data = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day_01"));
        let answer = running_increases::<1>(data);
        assert_eq!(answer, 1233);
    }
}

mod part_2 {
    use super::*;

    #[test]
    fn example() {
        let test_data = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
        assert_eq!(running_increases::<3>(test_data), 5);
    }

    #[test]
    fn answer() {
        let data = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day_01"));
        let answer = running_increases::<3>(data);
        assert_eq!(answer, 1275);
    }
}
