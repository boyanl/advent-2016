use std::cmp::{max, min};
use std::collections::btree_map::Values;
use std::io::stdin;
use std::ops::{Sub, SubAssign};
use std::vec;

use sscanf::sscanf;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct Interval {
    start: u32,
    end: u32,
}

fn interval(start: u32, end: u32) -> Interval {
    return Interval { start, end };
}

fn intersection(i1: Interval, i2: Interval) -> Interval {
    Interval {
        start: max(i1.start, i2.start),
        end: min(i1.end, i2.end),
    }
}
fn intersects(i1: Interval, i2: Interval) -> bool {
    !is_empty(intersection(i1, i2))
}

fn is_empty(i: Interval) -> bool {
    i.start > i.end
}

fn length(i: Interval) -> u32 {
    return i.end - i.start + 1;
}

impl Sub<Interval> for Interval {
    type Output = Interval;
    fn sub(self, rhs: Interval) -> Self::Output {
        let isect = intersection(self, rhs);
        if is_empty(isect) {
            return self;
        }
        assert!(isect.start == self.start || isect.end == self.end);
        if isect.start == self.start {
            return Interval {
                start: isect.end + 1,
                end: self.end,
            };
        }
        return Interval {
            start: self.start,
            end: isect.start - 1,
        };
    }
}

impl SubAssign<Interval> for Interval {
    fn sub_assign(&mut self, rhs: Interval) {
        *self = *self - rhs;
    }
}

// list of non-intersecting intervals in sorted order
type Intervals = Vec<Interval>;

fn compact(intervals: &mut Intervals) {
    let mut consecutive = Vec::new();
    let mut i = 0;
    while i < intervals.len() {
        let el = intervals[i];
        if i < intervals.len() - 1 && el.end + 1 == intervals[i + 1].start {
            let mut to = i + 1;
            while to < intervals.len() && intervals[to - 1].end + 1 == intervals[to].start {
                to += 1;
            }
            consecutive.push((i, to - 1));
            i = to;
        }
        i += 1;
    }

    for &(from, to) in consecutive.iter().rev() {
        intervals[from].end = intervals[to].end;
        for j in (from + 1..=to).rev() {
            intervals.remove(j);
        }
    }
}

fn add(intervals: &mut Intervals, val: Interval) {
    let mut current = val;
    let mut to_remove = Vec::new();
    for (i, el) in intervals.iter().enumerate() {
        let isect = intersection(*el, current);
        if !is_empty(isect) {
            if isect == current {
                return;
            } else if isect == *el {
                if el.start == current.start || el.end == current.end {
                    current -= isect;
                } else {
                    to_remove.push(i);
                }
            } else {
                current -= *el;
            }
        }
    }

    assert!(to_remove.is_empty() || !is_empty(current));

    for i in to_remove.iter().rev() {
        intervals.remove(*i);
    }
    if !is_empty(current) {
        intervals.push(current);
    }
    intervals.sort_by(|i1, i2| i1.start.cmp(&i2.start));
    compact(intervals);
}

fn normalized(interval_list: &Vec<Interval>) -> Intervals {
    let mut result = Vec::new();
    for interval in interval_list {
        add(&mut result, *interval);
    }

    result
}

fn read_input() -> Vec<Interval> {
    let mut result = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        if let Ok((start, end)) = sscanf!(&line, "{u32}-{u32}") {
            result.push(Interval { start, end });
        }
    }

    result
}

fn test_intervals() {
    let mut intervals: Vec<Interval> = Vec::new();
    add(&mut intervals, interval(3, 5));
    assert_eq!(intervals, vec![interval(3, 5)]);

    add(&mut intervals, interval(16, 20));
    assert_eq!(intervals, vec![interval(3, 5), interval(16, 20)]);

    add(&mut intervals, interval(4, 8));
    assert_eq!(intervals, vec![interval(3, 8), interval(16, 20)]);

    add(&mut intervals, interval(18, 25));
    assert_eq!(intervals, vec![interval(3, 8), interval(16, 25)]);

    add(&mut intervals, interval(0, 3));
    assert_eq!(intervals, vec![interval(0, 8), interval(16, 25)]);

    add(&mut intervals, interval(0, 3));
    assert_eq!(intervals, vec![interval(0, 8), interval(16, 25)]);
}

fn part_one() {
    let intervals = normalized(&read_input());
    let result = intervals[0].end + 1;
    println!("{result}");
}

fn part_two() {
    let intervals = normalized(&read_input());
    let total = u32::MAX as i64 + 1;
    let result = total - intervals.iter().map(|i| length(*i) as i64).sum::<i64>();
    println!("{result}");
}

fn main() {
    part_two();
}
