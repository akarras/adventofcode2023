use core::fmt::Debug;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    time::Instant,
};

pub struct TestRunner {
    run_test: &'static (dyn Fn() + Send + Sync + 'static),
    day: u8,
    part: u8,
}

impl TestRunner {
    pub fn run_test(day: u8, part: u8) {
        let test = inventory::iter::<TestRunner>()
            .find(|runner| runner.day == day && runner.part == part)
            .unwrap();
        (test.run_test)();
    }

    pub fn run_all() {
        let mut tests = inventory::iter::<TestRunner>
            .into_iter()
            .collect::<Vec<_>>();
        tests.sort_by_key(|test| (test.day, test.part));
        for test in tests {
            (test.run_test)();
        }
    }

    pub const fn new<D>() -> Self
    where
        D: DayPart + 'static,
    {
        Self {
            run_test: &run_test::<D>,
            day: D::DAY,
            part: D::PART,
        }
    }
}

inventory::collect!(TestRunner);

pub fn run_test<D>()
where
    D: DayPart,
{
    let start = Instant::now();
    println!("Day: {} Part: {}", D::DAY, D::PART);
    let lines = read_file(D::FILE);
    let result = D::run(lines);
    println!("{}", result);
    println!("Elapsed: {:?}", start.elapsed());
}
pub trait DayPart {
    const FILE: &'static str;
    const DAY: u8;
    const PART: u8;

    fn run(lines: impl Iterator<Item = String>) -> String;
}

pub trait ParseExt {
    fn read_delimited<'a, D>(&'a self, pattern: &'a str) -> impl Iterator<Item = D> + 'a
    where
        D: FromStr,
        D::Err: Debug;

    /// expects a tagged value
    /// ex: "Time: rest of the values" -> "rest of the values"
    fn expect_tag<'a>(&'a self, tag: &str) -> &'a str;
}

impl<T> ParseExt for T
where
    T: AsRef<str>,
{
    fn read_delimited<'a, D>(&'a self, pattern: &'a str) -> impl Iterator<Item = D> + 'a
    where
        D: FromStr,
        D::Err: Debug,
    {
        self.as_ref()
            .split(pattern)
            .filter(|p| !p.is_empty())
            .map(|p| p.parse().unwrap())
    }

    fn expect_tag<'a>(&'a self, tag: &str) -> &'a str {
        let (start, rest) = self.as_ref().trim().split_once(':').unwrap();
        assert_eq!(start, tag);
        rest
    }
}

pub fn read_file(file: &str) -> impl Iterator<Item = String> {
    let read = File::open(file).unwrap();
    let reader = BufReader::new(read);
    reader.lines().map(|l| l.unwrap())
}

pub struct TupleIter<I>(I)
where
    I: Iterator;

impl<I> Iterator for TupleIter<I>
where
    I: Iterator,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.0.next()?, self.0.next()?))
    }
}

pub trait IterExt
where
    Self: Iterator + Sized,
{
    // tuple groups
    fn tuple_pairs(self) -> TupleIter<Self>;
    fn count_distinct(self) -> DistinctCounter<Self>
    where
        Self: Iterator,
        <Self as Iterator>::Item: PartialEq + Eq;
}

impl<I> IterExt for I
where
    I: Iterator + Sized,
{
    fn tuple_pairs(self) -> TupleIter<Self> {
        TupleIter(self)
    }

    /// Requires a sorted set, but returns a list of distinct items
    fn count_distinct(self) -> DistinctCounter<Self>
    where
        I::Item: PartialEq + Eq,
    {
        DistinctCounter::new(self)
    }
}

pub struct DistinctCounter<I>
where
    I: Iterator,
    I::Item: PartialEq + Eq,
{
    iter: I,
    last_item: Option<I::Item>,
}
impl<I> DistinctCounter<I>
where
    I: Iterator + Sized,
    I::Item: PartialEq + Eq,
{
    fn new(iter: I) -> Self {
        Self {
            iter,
            last_item: None,
        }
    }
}

impl<I> Iterator for DistinctCounter<I>
where
    I: Iterator,
    I::Item: PartialEq + Eq,
{
    type Item = (usize, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let Self { iter, last_item } = self;
        let mut current_count = 1;
        while let Some(item) = iter.next() {
            if let Some(last) = last_item.take() {
                if last != item {
                    *last_item = Some(item);
                    let count = current_count;
                    return Some((count, last));
                } else {
                    current_count += 1;
                }
            }
            *last_item = Some(item);
        }
        if let Some(last_item) = self.last_item.take() {
            return Some((current_count, last_item));
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::IterExt;

    #[test]
    fn dedup() {
        let mut counts = "aabbcccdddde".chars().count_distinct().collect::<Vec<_>>();
        assert_eq!(
            counts,
            vec![(2, 'a'), (2, 'b'), (3, 'c'), (4, 'd'), (1, 'e')]
        );
    }
}
