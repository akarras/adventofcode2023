use core::fmt::Debug;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

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
        let (start, rest) = self.as_ref().trim().split_once(":").unwrap();
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
    type Item;
    fn tuple_pairs(self) -> TupleIter<Self>;
}

impl<I> IterExt for I
where
    I: Iterator + Sized,
{
    type Item = I::Item;

    fn tuple_pairs(self) -> TupleIter<Self> {
        TupleIter(self)
    }
}
