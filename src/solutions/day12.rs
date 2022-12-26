use std::{collections::VecDeque, str::FromStr};

type Coords = (usize, usize);

pub fn part1(input: &str) -> u32 {
    let graphe = Graphe::from_str(input).unwrap();

    let (start, goal) = {
        let mut start = (0, 0);
        let mut goal = (0, 0);

        for (y, line) in input.lines().enumerate() {
            if let Some(x) = line.chars().position(|c| c == 'S') {
                start = (x, y)
            }
            if let Some(x) = line.chars().position(|c| c == 'E') {
                goal = (x, y)
            }
        }

        (start, goal)
    };

    let distances = bfs(graphe, goal);

    distances[start.1][start.0].unwrap()
}

pub fn part2(input: &str) -> u32 {
    let graphe = Graphe::from_str(input).unwrap();

    let goal = {
        let mut goal = (0, 0);
        for (y, line) in input.lines().enumerate() {
            if let Some(x) = line.chars().position(|c| c == 'E') {
                goal = (x, y)
            }
        }
        goal
    };

    let distances = bfs(graphe, goal);

    let potential_starts = input
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.chars().enumerate().filter_map(move |(x, c)| {
                if c == 'a' || c == 'S' {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .flatten();

    // for line in distances.iter() {
    //     for h in line {
    //         print!("{:2} ", h / 10)
    //     }
    //     println!()
    // }

    // for (coord, d) in distances.iter().enumerate().map(|(y, row)| row.iter().enumerate().map(move |(x, d)| ((x, y), d))).flatten() {
    //     if *d == 0 {
    //         println!("{coord:?}")
    //     }
    // }

    potential_starts
        .filter_map(|(x, y)| distances[y][x])
        .min()
        .unwrap()
}

fn bfs(graphe: Graphe, start: Coords) -> Vec<Vec<Option<u32>>> {
    let mut distance = 0;
    const MARKER: (usize, usize) = (usize::MAX, usize::MAX);

    let mut distances = vec![vec![None; graphe.grid[0].len()]; graphe.grid.len()];

    let mut to_visit = VecDeque::new();
    to_visit.push_front(start);
    to_visit.push_front(MARKER);

    while let Some(cell) = to_visit.pop_back() {
        if to_visit.is_empty() {
            break;
        }

        if cell == MARKER {
            distance += 1;
            to_visit.push_front(MARKER);
            continue;
        }

        distances[cell.1][cell.0] = Some(distance);

        for neighbour in graphe.neighbour(cell) {
            if neighbour != start
                && distances[neighbour.1][neighbour.0] == None
                && !to_visit.contains(&neighbour)
            {
                to_visit.push_front(neighbour)
            }
        }
    }

    distances
}

struct Graphe {
    grid: Vec<Vec<u8>>,
}

impl Graphe {
    fn neighbour(&self, (x, y): Coords) -> Vec<Coords> {
        let mut neighbours = Vec::new();
        let current_heigth = self.grid[y][x];

        if x > 0 && self.grid[y][x - 1] + 1 >= current_heigth {
            neighbours.push((x - 1, y))
        }
        if x < self.grid[0].len() - 1 && self.grid[y][x + 1] + 1 >= current_heigth {
            neighbours.push((x + 1, y))
        }
        if y > 0 && self.grid[y - 1][x] + 1 >= current_heigth {
            neighbours.push((x, y - 1))
        }
        if y < self.grid.len() - 1 && self.grid[y + 1][x] + 1 >= current_heigth {
            neighbours.push((x, y + 1))
        }

        neighbours
    }
}

impl FromStr for Graphe {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|row| {
                row.chars()
                    .map(|c| match c {
                        'S' => 0,
                        'E' => 25,
                        c => c as u8 - 97,
                    })
                    .collect()
            })
            .collect();

        Ok(Self { grid })
    }
}
