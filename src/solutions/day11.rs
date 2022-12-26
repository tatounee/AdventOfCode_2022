use std::{collections::VecDeque, str::FromStr};

pub fn part1(input: &str) -> u64 {
    let mut monkeys = input
        .split("\r\n\r\n")
        .map(|monkey| Monkey::from_str(monkey.split_once(':').unwrap().1.trim()).unwrap())
        .collect::<Vec<_>>();
    let total_monkey = monkeys.len();

    for _ in 0..20 {
        for i in 0..total_monkey {
            let monkey = &mut monkeys[i];
            let throws = monkey.inspect_1();
            for (idx, item) in throws {
                monkeys[idx].catch(item)
            }
        }
    }
    monkeys
        .sort_unstable_by(|monkey1, monkey2| monkey2.inspected_count.cmp(&monkey1.inspected_count));
    (monkeys[0].inspected_count * monkeys[1].inspected_count) as u64
}

pub fn part2(input: &str) -> u64 {
    let mut monkeys = input
        .split("\r\n\r\n")
        .map(|monkey| Monkey::from_str(monkey.split_once(':').unwrap().1.trim()).unwrap())
        .collect::<Vec<_>>();
    let total_monkey = monkeys.len();

    // We don't really calculate the PPCM but a multiple of the PPCm but that change nothing
    let ppcm = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product();

    dbg!(ppcm * ppcm);

    for _ in 0..10_000 {
        for i in 0..total_monkey {
            let monkey = &mut monkeys[i];
            let throws = monkey.inspect_2(ppcm);
            for (idx, item) in throws {
                monkeys[idx].catch(item)
            }
        }
    }
    monkeys
        .sort_unstable_by(|monkey1, monkey2| monkey2.inspected_count.cmp(&monkey1.inspected_count));
    (monkeys[0].inspected_count * monkeys[1].inspected_count) as u64
}

struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    inspected_count: usize,
}

impl Monkey {
    fn inspect_1(&mut self) -> Vec<(usize, u64)> {
        self.inspected_count += self.items.len();

        self.items
            .drain(..)
            .map(|mut item| {
                item = self.operation.execute(item) / 3;

                (self.test.test(item), item)
            })
            .collect()
    }
    fn inspect_2(&mut self, modulo: u64) -> Vec<(usize, u64)> {
        self.inspected_count += self.items.len();

        self.items
            .drain(..)
            .map(|mut item| {
                item = self.operation.execute(item) % modulo;

                (self.test.test(item), item)
            })
            .collect()
    }

    #[inline]
    fn catch(&mut self, item: u64) {
        self.items.push_back(item)
    }
}

struct Operation {
    left: Variable,
    op: Op,
    rigth: Variable,
}

impl Operation {
    fn new(left: Variable, op: Op, rigth: Variable) -> Self {
        Self { left, op, rigth }
    }

    fn execute(&self, x: u64) -> u64 {
        let left = match self.left {
            Old => x,
            Number(n) => n,
        };

        let rigth = match self.rigth {
            Old => x,
            Number(n) => n,
        };

        match self.op {
            Mul => left * rigth,
            Add => left + rigth,
        }
    }
}

struct Test {
    divisible_by: u64,
    success: usize,
    failure: usize,
}

impl Test {
    fn test(&self, x: u64) -> usize {
        if x % self.divisible_by == 0 {
            self.success
        } else {
            self.failure
        }
    }
}

use Variable::*;
#[derive(Debug, Clone, Copy)]
enum Variable {
    Number(u64),
    Old,
}

use Op::*;
#[derive(Debug, Clone, Copy)]
enum Op {
    Mul,
    Add,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (starting_items, s) = s.split_once("\r\n").unwrap();
        let (operation, test) = s.split_once("\r\n").unwrap();

        let starting_items = starting_items.trim()[16..]
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();

        let operation = operation.trim()[17..].split(' ').collect::<Vec<_>>();

        let operation = Operation::new(
            Variable::from_str(operation[0])?,
            Op::from_str(operation[1])?,
            Variable::from_str(operation[2])?,
        );

        let test = Test::from_str(test.trim())?;

        Ok(Self {
            items: starting_items,
            operation,
            test,
            inspected_count: 0,
        })
    }
}

impl FromStr for Variable {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Old),
            s => s.parse().map(|n| Number(n)).map_err(|_| ()),
        }
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Add),
            "*" => Ok(Mul),
            _ => Err(()),
        }
    }
}

impl FromStr for Test {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let test = s.lines().collect::<Vec<_>>();
        let divisible_by = test[0].trim()[19..].parse().unwrap();
        let success = test[1].trim()[25..].parse().unwrap();
        let failure = test[2].trim()[26..].parse().unwrap();

        Ok(Self {
            divisible_by,
            success,
            failure,
        })
    }
}
