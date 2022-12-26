use crate::utils::abs_range_inclusive;

pub fn part1(input: &str) -> u32 {
    let mut rocks = parse_rocks(input);

    let (mut min_x, mut max_x, max_y) = find_boundary(&rocks);
    min_x -= 1;
    max_x += 1;
    max_x -= min_x;

    let mut cave = vec![vec![Empty; max_y + 1]; max_x]; // cave[x][y]

    rocks
        .iter_mut()
        .for_each(|rock| rock.iter_mut().for_each(|(x, _)| *x -= min_x));

    for rock in rocks {
        trace_rock(&mut cave, &rock);
    }

    let spawing_sand_x = 500 - min_x;
    simulate_falling_sand(spawing_sand_x, cave, |y, _| y == max_y)
}

pub fn part2(input: &str) -> u32 {
    let mut rocks = parse_rocks(input);

    let max_y = find_boundary(&rocks).2 + 2;
    let shift = 500 - max_y;

    let mut cave = vec![vec![Empty; max_y + 1]; 2 * max_y + 1]; // cave[x][y]
    for column in cave.iter_mut().take(2 * max_y + 1) {
        column[max_y] = Rock
    }

    rocks
        .iter_mut()
        .for_each(|rock| rock.iter_mut().for_each(|(x, _)| *x -= shift));

    for rock in rocks {
        trace_rock(&mut cave, &rock);
    }

    let spawing_sand_x = max_y;
    simulate_falling_sand(spawing_sand_x, cave, |_, cave| {
        cave[spawing_sand_x][0] == Sand
    })
}

fn simulate_falling_sand(
    spawing_sand_x: usize,
    mut cave: Vec<Vec<Cell>>,
    predicat: impl Fn(usize, &[Vec<Cell>]) -> bool,
) -> u32 {
    for i in 0.. {
        let (mut x, mut y) = (spawing_sand_x, 0);
        loop {
            if predicat(y, &cave) {
                return i;
            }

            if cave[x][y + 1].is_empty() {
                y += 1;
            } else if cave[x - 1][y + 1].is_empty() {
                x -= 1;
                y += 1;
            } else if cave[x + 1][y + 1].is_empty() {
                x += 1;
                y += 1
            } else {
                cave[x][y] = Sand;
                break;
            }
        }
    }

    unreachable!()
}

fn parse_rocks(input: &str) -> Vec<Vec<(usize, usize)>> {
    input
        .lines()
        .map(|rock| {
            rock.split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(',').unwrap();
                    (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

use Cell::*;
#[derive(Clone, PartialEq)]
enum Cell {
    Empty,
    Rock,
    Sand,
}

impl Cell {
    const fn is_empty(&self) -> bool {
        matches!(self, Empty)
    }

    const fn is_solid(&self) -> bool {
        !matches!(self, Empty)
    }
}

// min_x, mut max_x, max_y
fn find_boundary(rocks: &[Vec<(usize, usize)>]) -> (usize, usize, usize) {
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;

    for (x, y) in rocks.iter().flatten() {
        if *x < min_x {
            min_x = *x
        }
        if *x > max_x {
            max_x = *x
        }
        if *y > max_y {
            max_y = *y
        }
    }

    (min_x, max_x, max_y)
}

fn trace_rock(cave: &mut [Vec<Cell>], rock: &[(usize, usize)]) {
    for [(x1, y1), (x2, y2)] in rock.array_windows() {
        if x1 == x2 {
            for y in abs_range_inclusive(*y1, *y2) {
                cave[*x1][y] = Rock
            }
        } else {
            for x in abs_range_inclusive(*x1, *x2) {
                cave[x][*y1] = Rock
            }
        }
    }
}
