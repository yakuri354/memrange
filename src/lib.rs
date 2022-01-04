#![no_std]
extern crate alloc;

use core::cmp;
use core::cmp::Ordering;
use core::u64;
use alloc::boxed::Box;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct Range {
    pub min: u64,
    pub max: u64,
}

pub fn range(min: u64, max: u64) -> Range {
    Range::new(min, max)
}

impl Range {
    pub fn new(min: u64, max: u64) -> Range {
        assert!(min <= max);
        return Range { min, max };
    }

    pub fn intersect(&self, other: &Range) -> bool {
        cmp::max(self.min, other.min) <= cmp::min(self.max, other.max)
    }

    pub fn adjacent(&self, other: &Range) -> bool {
        let right_of = self.min > other.max && self.min == other.max + 1;
        let left_of = self.max < other.min && self.max + 1 == other.min;
        return self.intersect(other) || right_of || left_of;
    }

    pub fn get_intersection(&self, other: &Range) -> Range {
        return Range::new(cmp::max(self.min, other.min), cmp::min(self.max, other.max));
    }

    pub fn get_union(&self, other: &Range) -> Range {
        return Range::new(cmp::min(self.min, other.min), cmp::max(self.max, other.max));
    }

    pub fn get_difference(&self, other: &Range) -> (Option<Range>, Option<Range>) {
        let mut first_part = None;
        let mut last_part = None;
        if (self.min <= other.min) && (other.min > 0) {
            first_part = Some(Range::new(self.min, other.min));
        }
        if (other.max <= self.max) && (other.max < u64::MAX) {
            last_part = Some(Range::new(other.max, self.max));
        }
        return (first_part, last_part);
    }

    pub fn len(&self) -> u64 {
        return self.max - self.min + 1;
    }

    pub fn get_extended(&self) -> Range {
        let mut res = self.clone();
        if res.min > 0 {
            res.min -= 1
        }
        if res.max < !0 {
            res.max += 1
        }
        return res;
    }

    ///```
    ///use memrange::{range, Range};
    ///assert_eq!(range(0,5).each_slice(2).collect::<Vec<Range>>(), vec![range(0,1), range(2,3), range(4,5)])
    ///```
    pub fn each_slice(&self, size: u64) -> Box<dyn Iterator<Item=Range>> {
        let num_results = self.len() / size;
        let min = self.min;
        let max = self.max;
        let iter = (0..num_results).map(move |i| range(min + i * size, cmp::min(min + (i + 1) * size - 1, max)));
        return Box::new(iter);
    }

    ///```
    ///use memrange::{range, Range};
    ///use std::cmp::Ordering;
    ///assert_eq!(range(  0,5).intersection_cmp(&range(6,100)), Ordering::Less);
    ///assert_eq!(range(  0,5).intersection_cmp(&range(5,100)), Ordering::Equal);
    ///assert_eq!(range(5,100).intersection_cmp(&range(1,2)),   Ordering::Greater);
    ///```
    ///returns Equal if self and b are intersecting, the usual comparision otherwise
    pub fn intersection_cmp(&self, b: &Range) -> Ordering {
        if self.intersect(b) {
            return Ordering::Equal;
        }
        if self < b {
            return Ordering::Less;
        }
        if self > b {
            return Ordering::Greater;
        }
        unreachable!();
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        let first_cmp = self.min.cmp(&other.min);
        if first_cmp == Ordering::Equal { return self.max.cmp(&other.max); }
        return first_cmp;
    }
}
