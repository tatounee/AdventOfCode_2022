use std::{cmp::Ordering, iter::zip, str::FromStr};

pub fn part1(input: &str) -> usize {
    let pairs = input.split("\r\n\r\n").map(|pair| {
        let (left, right) = pair.trim().split_once("\r\n").unwrap();
        (
            Signal::from_str(left).unwrap(),
            Signal::from_str(right).unwrap(),
        )
    });

    pairs
        .enumerate()
        .filter_map(
            |(i, (left, right))| {
                if left < right {
                    Some(i + 1)
                } else {
                    None
                }
            },
        )
        .sum()
}

pub fn part2(input: &str) -> usize {
    let divider_1: Signal = List(vec![List(vec![Number(2)])]);
    let divider_2: Signal = List(vec![List(vec![Number(6)])]);

    let mut signals = input
        .lines()
        .filter_map(|line| Signal::from_str(line).ok())
        .collect::<Vec<_>>();
    signals.push(divider_1.clone());
    signals.push(divider_2.clone());

    signals.sort_unstable();

    let index_1 = signals
        .iter()
        .position(|signal| signal == &divider_1)
        .unwrap()
        + 1;
    let index_2 = signals
        .iter()
        .position(|signal| signal == &divider_2)
        .unwrap()
        + 1;

    index_1 * index_2
}

use Signal::*;
#[derive(Debug, Clone, PartialEq, Eq)]
enum Signal {
    List(Vec<Signal>),
    Number(u32),
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (List(list1), List(list2)) => {
                for (left, right) in zip(list1, list2) {
                    if let Some(local_cmp) = left.partial_cmp(right) && local_cmp != Ordering::Equal {
                        return Some(local_cmp);
                    }
                }

                list1.len().partial_cmp(&list2.len())
            }
            (list @ List(_), number @ Number(_)) => list.partial_cmp(&List(vec![number.clone()])),
            (number @ Number(_), list @ List(_)) => List(vec![number.clone()]).partial_cmp(list),
            (Number(a), Number(b)) => a.partial_cmp(b),
        }
    }
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Signal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }
        if s == "[]" {
            return Ok(List(vec![]));
        }
        if let Ok(n) = s.parse::<u32>() {
            return Ok(Number(n));
        }

        let mut signals = Vec::new();
        let mut parsing_start = 1;
        let mut depth = 0;

        for i in 0..s.len() {
            match s.as_bytes()[i] {
                b'[' => depth += 1,
                b']' => depth -= 1,
                b',' => {
                    if depth == 1 {
                        signals.push(Signal::from_str(&s[parsing_start..i])?);
                        parsing_start = i + 1
                    }
                }
                _ => (),
            }
        }
        signals.push(Signal::from_str(&s[parsing_start..(s.len() - 1)])?);

        Ok(List(signals))
    }
}

// #[test]
// #[allow(unused_variables)]
// fn parsing() {
//     let s1 =
//         "[[[],10,[],8,10],[[[],7,0,[10,0,1,10,4]],5],[8,[4,3],[[10,10,9,1],[3,0,6]],4],[[]],[]]";
//     let s2 = "[]";
//     let s3 = "[6,4]";
//     let s4 = "[0,[]]";
//     let s = Signal::from_str(s1).unwrap();
//     println!("{s:?}")
// }

// #[test]
// fn comparasion() {
//     let s1 = Signal::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap();
//     let s2 = Signal::from_str("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap();

//     println!("{:?}", s1.cmp(&s2))
// }
