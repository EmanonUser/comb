use std::time::Instant;
use rayon::prelude::*;

#[derive(Debug)]
struct CartesianProductIterator<'a> {
    data: &'a [char],
    length: usize,
    current_indices: Vec<usize>,
    is_finished: bool,
}

impl<'a> CartesianProductIterator<'a> {
    fn new(data: &'a [char], length: usize) -> Self {
        let current_indices = vec![0; length];
        let is_finished = length == 0;
        CartesianProductIterator {
            data,
            length,
            current_indices,
            is_finished,
        }
    }

    fn increment_indices(&mut self) {
        for i in (0..self.length).rev() {
            if self.current_indices[i] < self.data.len() - 1 {
                self.current_indices[i] += 1;
                return;
            } else {
                self.current_indices[i] = 0;
            }
        }
        self.is_finished = true;
    }
}

impl<'a> Iterator for CartesianProductIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished {
            return None;
        }
        let combination = self
            .current_indices
            .iter()
            .map(|&index| self.data[index])
            .collect::<String>();

        self.increment_indices();

        Some(combination)
    }
}

fn cartesian_product(data: &[char], length: usize) -> CartesianProductIterator {
    CartesianProductIterator::new(data, length)
}

fn main() {
    let data = ('a'..='z').collect::<Vec<_>>();
    let start = Instant::now();

    let r = cartesian_product(&data, 5).count();
    let duration = start.elapsed();
    println!("duration time {:?}", duration);
    println!("count {:?}", r);
}
