use std::{env, io};
use std::collections::VecDeque;
use std::ops::{Index, IndexMut};
use std::time::Instant;

use rand::thread_rng;
use rand::seq::SliceRandom;
use len_trait::Len;
use std::fs::File;
use std::io::Write;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: vec_vs_deque num_values num_experiments (for|fold)");
    } else {
        let num_values = args[1].parse::<usize>().unwrap();
        let num_experiments = args[2].parse::<usize>().unwrap();
        let mut vec = Vec::new();
        let mut deque = Vec::new();
        for i in 1..num_experiments+1 {
            let mut numbers: Vec<usize> = (0..num_values).collect();
            numbers.shuffle(&mut thread_rng());
            vec.push(test(&mut Vec::new(), &numbers));
            deque.push(test(&mut VecDeque::new(), &numbers));
            println!("Experiment {}: vec: {} ms deque: {} ms", i, vec.last().unwrap(), deque.last().unwrap());
        }

        let mut output = File::create(format!("results_{}.csv", num_values))?;
        write!(output, "vec,deque\n")?;
        for i in 0..num_experiments {
            write!(output, "{},{}\n", vec[i], deque[i])?;
        }
    }
    Ok(())
}

fn test(v: &mut dyn VTest<usize,Output=usize>, nums: &Vec<usize>) -> u128 {
    let start = Instant::now();
    nums.iter().for_each(|n| v.add(*n));
    v.selection_sort();
    Instant::now().duration_since(start).as_millis()
}

trait VTest<T:Copy+PartialEq+PartialOrd+Sized> : Index<usize,Output=T> + IndexMut<usize,Output=T> + Len {
    fn add(&mut self, t: T);

    fn swap(&mut self, i: usize, j: usize) {
        let temp: T = self[i];
        self[i] = self[j];
        self[j] = temp;
    }

    fn selection_sort(&mut self) {
        for i in 0..self.len() {
            let lowest = (i+1..self.len())
                .fold(i, |lowest, j|
                    if self[lowest] < self[j] {lowest} else {j});
            self.swap(lowest, i);
        }
    }
}

impl <T:Copy+PartialEq+PartialOrd+Sized> VTest<T> for Vec<T> {
    fn add(&mut self, t: T) {
        self.push(t);
    }
}

impl <T:Copy+PartialEq+PartialOrd+Sized> VTest<T> for VecDeque<T> {
    fn add(&mut self, t: T) {
        self.push_back(t);
    }
}
