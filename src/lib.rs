// Author: blackprince001
// Implementation of Python Counter Module
// https://docs.python.org/3/library/collections.html?highlight=counter#collections.Counter

use std::{collections::HashMap, panic};

#[derive(PartialEq, Eq, Clone)]
pub struct Counter<
    T: std::cmp::Eq + std::hash::Hash + std::marker::Copy + std::cmp::PartialOrd + std::cmp::PartialEq,
> {
    table: HashMap<T, usize>,
}

impl<
        T: std::cmp::Eq
            + std::hash::Hash
            + std::marker::Copy
            + std::cmp::PartialOrd
            + std::cmp::PartialEq,
    > Default for Counter<T>
{
    fn default() -> Self {
        Counter {
            table: HashMap::new(),
        }
    }
}

impl<
        T: std::cmp::Eq
            + std::hash::Hash
            + std::marker::Copy
            + std::cmp::PartialOrd
            + std::cmp::PartialEq,
    > Counter<T>
{
    pub fn new() -> Self {
        Counter {
            table: HashMap::new(),
        }
    }

    pub fn from(mut self, iterable: impl IntoIterator<Item = T> + std::clone::Clone) -> Counter<T> {
        for item in iterable.clone().into_iter() {
            if self.table.contains_key(&item) {
                *self.table.get_mut(&item).unwrap() += 1;
                continue;
            }
            self.table.insert(item, 1);
        }

        Counter { table: self.table }
    }

    pub fn get(&self, key: T) -> Option<usize> {
        if self.table.contains_key(&key) {
            self.table.get(&key).copied()
        } else {
            Some(0)
        }
    }

    pub fn update(&mut self, key: T, count: usize) {
        self.table.insert(key, count);
    }

    pub fn elements(&self) -> impl IntoIterator<Item = T> {
        let mut vector = vec![];
        for (item, count) in &self.table {
            for _ in 0..*count {
                vector.push(*item);
            }
        }

        vector.into_iter()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, &usize)> {
        self.table.iter()
    }

    pub fn most_common(&self, n: usize) -> Vec<(&T, &usize)> {
        if n > self.table.keys().len() {
            panic!("{n} Exceeds the number of unique items in our Counter Struct.")
        }

        let mut iter = self.table.iter().collect::<Vec<_>>();
        let mut slice = vec![];

        // sort by descending order with counts from each
        iter.sort_by(|a, b| (b.1).partial_cmp(a.1).unwrap());
        iter[..n].clone_into(&mut slice);

        slice
    }

    pub fn substract(&self, other: &Self) -> Counter<T> {
        let mut ctmp = Counter::new();

        for (self_key, self_count) in &self.table {
            for (other_key, other_count) in &other.table {
                if self_key == other_key {
                    ctmp.update(
                        *self_key,
                        if self_count > other_count {
                            *self_count - *other_count
                        } else {
                            *other_count - *self_count
                        },
                    )
                }
            }
        }
        ctmp
    }

    pub fn total(&self) -> usize {
        let total_count: usize = self.table.values().sum();

        total_count
    }

    pub fn clear(&mut self) {
        self.table.clear()
    }
}
