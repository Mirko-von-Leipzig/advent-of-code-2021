//! Contains useful iterators

use std::collections::VecDeque;

pub trait WindowIterator<I, T>
where
    I: Iterator<Item = T>,
    T: Copy + std::fmt::Debug + Default,
{
    fn window<const N: usize>(self) -> WindowIter<I, T, N>;
}

pub struct WindowIter<I, T, const N: usize>
where
    I: Iterator<Item = T>,
    T: Copy + std::fmt::Debug + Default,
{
    iterator: I,
    buffer: VecDeque<T>,
    init: bool,
}

impl<I, T, const N: usize> Iterator for WindowIter<I, T, N>
where
    I: Iterator<Item = T>,
    T: Copy + std::fmt::Debug + Default,
{
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        if !self.init {
            self.init = true;
            for _ in 0..N - 1 {
                match self.iterator.next() {
                    Some(item) => self.buffer.push_back(item),
                    None => return None,
                }
            }
        } else {
            self.buffer.pop_front();
        }

        match self.iterator.next() {
            Some(item) => self.buffer.push_back(item),
            None => return None,
        }

        let mut result = [T::default(); N];
        #[allow(clippy::manual_memcpy)]
        // reason = "clippy bug: https://github.com/rust-lang/rust-clippy/issues/8160"
        for i in 0..N {
            result[i] = self.buffer[i];
        }

        Some(result)
    }
}

impl<I, T> WindowIterator<I, T> for I
where
    I: Iterator<Item = T>,
    T: Copy + std::fmt::Debug + Default,
{
    fn window<const N: usize>(self) -> WindowIter<I, T, N> {
        WindowIter {
            iterator: self,
            buffer: VecDeque::with_capacity(N),
            init: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod window_iter {
        use super::*;

        #[test]
        fn empty() {
            let mut pair_iter = std::iter::empty::<u16>().window::<2>();
            assert_eq!(pair_iter.next(), None);
        }

        #[test]
        fn single_element() {
            let mut pair_iter = (0..1).window::<2>();
            assert_eq!(pair_iter.next(), None);
        }

        #[test]
        fn pais() {
            let mut pair_iter = (0..=2).window::<2>();
            assert_eq!(pair_iter.next(), Some([0, 1]));
            assert_eq!(pair_iter.next(), Some([1, 2]));
            assert_eq!(pair_iter.next(), None);
        }

        #[test]
        fn triplets() {
            let mut triplet_iter = (0..=2).window::<3>();
            assert_eq!(triplet_iter.next(), Some([0, 1, 2]));
            assert_eq!(triplet_iter.next(), None);
        }
    }
}
