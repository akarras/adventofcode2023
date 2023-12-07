use std::{iter::Enumerate, ops::RangeInclusive};

use advent::advent_of_code;
use advent_utils::*;

#[advent_of_code(day = 3, part = 1)]
fn day_3_part_1(lines: impl Iterator<Item = String>) -> String {
    Grid::from_lines_owned(lines)
        .get_adjacent_numbers()
        .sum::<u32>()
        .to_string()
}

#[advent_of_code(day = 3, part = 2)]
fn day_3_part_2(lines: impl Iterator<Item = String>) -> String {
    Grid::from_lines_owned(lines).find_gear_ratios().to_string()
}

/// Yields a range from an iterator
struct RangeIter<I, T>
where
    I: Iterator,
{
    iter: Enumerate<I>,
    pred: T,
}

trait RangeI {
    fn get_ranges<T>(self, predicate: T) -> RangeIter<Self, T>
    where
        Self: Iterator + Sized,
        T: Fn(Self::Item) -> bool;
}

impl<I> RangeI for I
where
    I: Iterator,
{
    fn get_ranges<T>(self, predicate: T) -> RangeIter<Self, T>
    where
        Self: Iterator + Sized,
        T: Fn(I::Item) -> bool,
    {
        RangeIter {
            iter: self.enumerate(),
            pred: predicate,
        }
    }
}

impl<I, T> Iterator for RangeIter<I, T>
where
    I: Iterator,
    T: Fn(I::Item) -> bool,
{
    type Item = RangeInclusive<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { iter, pred } = self;
        let mut start_index = None;
        let mut last_index = None;
        for (index, item) in iter {
            if pred(item) {
                if start_index.is_none() {
                    start_index = Some(index);
                }
                last_index = Some(index);
            } else if let (Some(start_index), Some(last_index)) = (start_index, last_index) {
                return Some(start_index..=last_index);
            }
        }
        start_index.and_then(|start| last_index.map(|last| start..=last))
    }
}

struct Grid {
    rows: Box<[Box<[u8]>]>,
}

impl Grid {
    fn get_row_col(&self, row: i32, col: i32) -> Option<u8> {
        self.rows
            .get(usize::try_from(row).ok()?)?
            .get(usize::try_from(col).ok()?)
            .copied()
    }

    fn get_ajacent(&self, row: i32, col: i32) -> [Option<u8>; 9] {
        let kernel = [
            (row - 1, col - 1),
            (row - 1, col),
            (row - 1, col + 1),
            (row, col - 1),
            (row, col),
            (row, col + 1),
            (row + 1, col - 1),
            (row + 1, col),
            (row + 1, col + 1),
        ];
        kernel.map(|(row, col)| self.get_row_col(row, col))
    }

    fn check_adjacent_predicate<T>(&self, row: usize, col: usize, pred: T) -> bool
    where
        T: Fn(u8) -> bool + 'static + Copy,
    {
        self.get_ajacent(row as i32, col as i32)
            .into_iter()
            .any(|f| f.map(pred).unwrap_or(false))
    }

    fn iterate_numbers(&self) -> impl Iterator<Item = (usize, RangeInclusive<usize>)> + '_ {
        self.rows
            .iter()
            .enumerate()
            .flat_map(|(row_index, characters)| {
                characters
                    .iter()
                    .get_ranges(|char| char.is_ascii_digit())
                    .map(move |range| (row_index, range))
            })
    }

    fn get_number(&self, row: usize, columns: RangeInclusive<usize>) -> Option<u32> {
        let chars = self.rows.get(row)?.get(columns)?;
        let mut position = 1;
        let mut acc = 0;
        for char in chars.iter().rev() {
            let char = *char as char;
            let digit = char.to_digit(10).unwrap();
            acc += digit * position;
            position *= 10;
        }
        Some(acc)
    }

    fn get_adjacent_numbers(&self) -> impl Iterator<Item = u32> + '_ {
        self.iterate_numbers()
            .filter(|(row, col)| {
                col.clone().into_iter().any(|col| {
                    self.check_adjacent_predicate(*row, col, |c| !(c.is_ascii_digit() || c == b'.'))
                })
            })
            .map(|(row, col)| self.get_number(row, col).unwrap())
    }

    fn find_gear_ratios(&self) -> u32 {
        let numbers: Vec<_> = self.iterate_numbers().collect();
        let numbers = &numbers;
        self.rows
            .iter()
            .enumerate()
            .flat_map(|(row_i, cols)| {
                cols.iter()
                    .copied()
                    .enumerate()
                    .filter_map(move |(col_i, value)| {
                        (value == b'*')
                            .then(|| {
                                let numbers: Vec<_> = numbers
                                    .iter()
                                    .filter(move |(row, cols)| {
                                        row.abs_diff(row_i) <= 1
                                            && (cols.start().abs_diff(col_i) <= 1
                                                || cols.end().abs_diff(col_i) <= 1)
                                    })
                                    .flat_map(move |(row, columns)| {
                                        self.get_number(*row, columns.clone())
                                    })
                                    .collect();
                                if numbers.len() == 2 {
                                    Some(numbers.into_iter().product::<u32>())
                                } else {
                                    None
                                }
                            })
                            .flatten()
                    })
            })
            .sum::<u32>()
    }

    fn from_lines_owned(iter: impl Iterator<Item = String>) -> Self {
        Self {
            rows: iter
                .map(|grid| grid.trim().as_bytes().to_owned().into_boxed_slice())
                .collect::<Box<[_]>>(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Grid;
    fn from_lines<'a>(iter: impl Iterator<Item = &'a str>) -> Grid {
        Grid {
            rows: iter
                .map(|grid| grid.trim().as_bytes().to_owned().into_boxed_slice())
                .collect::<Box<[_]>>(),
        }
    }
    #[test]
    fn parse() {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        let grid = from_lines(input.lines());
        assert_eq!(grid.get_number(0, 0..=2), Some(467));
        assert_eq!(grid.iterate_numbers().next(), Some((0, 0..=2)));
        let numbers = grid
            .iterate_numbers()
            .map(|(row, cols)| grid.get_number(row, cols).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            numbers,
            vec![467, 114, 35, 633, 617, 58, 592, 755, 664, 598]
        );

        assert_eq!(grid.find_gear_ratios(), 467835);
        // assert_eq!(
        //     grid.get_non_adjacent_numbers().collect::<Vec<_>>(),
        //     vec![114, 58]
        // );
    }
}
