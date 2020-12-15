use itertools::FoldWhile::Continue;
use itertools::FoldWhile::Done;
use std::collections::VecDeque;
use crate::constants;
use itertools::Itertools;

const XMAS_CYPHER: &str = constants::DAY_NINE;

#[derive(Debug, Clone)]
struct RollingCypherList {
    queue: VecDeque<usize>,
    size: usize,
}

impl RollingCypherList {
    fn new (numbers: &[usize], size: usize) -> Self {
        let mut queue: VecDeque<usize> = VecDeque::with_capacity(size);
        for number in numbers.into_iter() {
            queue.push_back(*number);
        }
        Self {
            queue,
            size,
        }
    }

    pub fn push(&mut self, num: &usize) {
        self.queue.pop_front();
        self.queue.push_back(*num);
    }

    pub fn check_valid(&self, num: usize) -> bool {
        let mut is_valid = false;

        for i in 0..self.size {
            for j in i..self.size {
                if self.queue.get(i).unwrap()+self.queue.get(j).unwrap() ==num {
                    is_valid = true;
                }
            }
        }

        is_valid
    }

}

pub fn find_vulnerability() -> usize {
    let mut list: Vec<usize> = XMAS_CYPHER.lines()
        .map(|l| l.parse().unwrap())
        .collect();
    
    let mut queue = RollingCypherList::new(&list[0..25], 25);
    list.drain(0..25);
    loop {
        if list.len() == 0 {
            panic!("could not find a match!");
        }

        // check if current value is valid 
        if queue.check_valid(list[0]) {
            // update the rolling list with the new valid element and keep going
            queue.push(&list.remove(0));

        } else {
            // not valid, return here
            return list[0];
        }

    }
}

pub fn find_encryption_weakness(num: usize) -> usize {
    let mut list: Vec<usize> = XMAS_CYPHER.lines()
        .map(|l| l.parse().unwrap())
        .collect();

    for i in 0..list.len() {
        let end_index = list[i..list.len()].iter()
        .enumerate()
        // need to exit early with the value of the index, which is why we need fold_while
        .fold_while(Some(0), |acc, current: (usize, &usize)| {
            match acc {
                Some(n) => {
                    if n == num {
                        // found the end index of the sum range
                        return Done(Some(current.0));
                    }
                    let sum = n + *current.1;
                    if sum > num {
                        // our current range is greater than target, exit fold
                        return Done(None);
                    }
                    // we havent hit target or gone over. continue
                    Continue(Some(sum))
                },
                // insurance to exit when we are over
                None => Done(None)
            }

        });

        match end_index{
            Done(end) => {
                match end {
                    Some(e) => {
                        // Done/Some represents we exited early AND found a match, perform logic.
                        let sum_range = &list[i..i+e];
                        let min = sum_range.iter().min().unwrap();
                        let max = sum_range.iter().max().unwrap();

                        return min + max;
                    },
                    None => {}
                }
            },
            _ => {}
        }
    }

    // return a default value
    0
}
