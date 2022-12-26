
use std::str::FromStr;

pub fn part1(_input: &str) -> u32 {
    0
}

#[allow(clippy::needless_range_loop)]
pub fn part2(input: &str) -> u32 {
    let forest = Forest::from_str(input).unwrap();
    let mut scenic_score = vec![vec![1; forest.width()]; forest.height()];

    // Look up
    let mut number_of_visible_trees = vec![vec![0; forest.width()]; forest.height()];
    for x in 0..forest.width() {
        let mut last_seen = LastSeen::new();
        for y in 0..forest.height() {
            let tree = forest.get(x, y).unwrap();

            let mut maximum_position = 0;
            for (last_height, position) in last_seen.iter() {
                if last_height >= tree {
                    maximum_position = *position;
                    break;
                }
            }
            last_seen.see(*tree, y);

            number_of_visible_trees[y][x] += y - maximum_position
        }
    }
    multiply(&mut scenic_score, &number_of_visible_trees);

    // Look down
    let mut number_of_visible_trees = vec![vec![0; forest.width()]; forest.height()];
    for x in 0..forest.width() {
        let mut last_seen = LastSeen::new();
        for y in (0..forest.height()).rev() {
            let tree = forest.get(x, y).unwrap();

            let mut maximun_position = forest.height() - 1;
            for (last_height, position) in last_seen.iter() {
                if last_height >= tree {
                    maximun_position = *position;
                    break;
                }
            }
            last_seen.see(*tree, y);

            number_of_visible_trees[y][x] += maximun_position - y
        }
    }
    multiply(&mut scenic_score, &number_of_visible_trees);

    // Look left
    let mut number_of_visible_trees = vec![vec![0; forest.width()]; forest.height()];
    for y in 0..forest.height() {
        let mut last_seen = LastSeen::new();
        for x in 0..forest.width() {
            let tree = forest.get(x, y).unwrap();

            let mut maximun_position = 0;
            for (last_height, position) in last_seen.iter() {
                if last_height >= tree {
                    maximun_position = *position;
                    break;
                }
            }
            last_seen.see(*tree, x);

            number_of_visible_trees[y][x] += x - maximun_position
        }
    }
    multiply(&mut scenic_score, &number_of_visible_trees);

    // Look rigth
    let mut number_of_visible_trees = vec![vec![0; forest.width()]; forest.height()];
    for y in 0..forest.height() {
        let mut last_seen = LastSeen::new();
        for x in (0..forest.width()).rev() {
            let tree = forest.get(x, y).unwrap();

            let mut maximun_position = forest.width() - 1;
            for (last_height, position) in last_seen.iter() {
                if last_height >= tree {
                    maximun_position = *position;
                    break;
                }
            }
            last_seen.see(*tree, x);

            number_of_visible_trees[y][x] += maximun_position - x
        }
    }
    multiply(&mut scenic_score, &number_of_visible_trees);

    *scenic_score
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap() as u32
}

struct Forest {
    forest: Vec<Vec<u8>>,
}

impl Forest {
    #[inline]
    fn width(&self) -> usize {
        self.forest[0].len()
    }

    #[inline]
    fn height(&self) -> usize {
        self.forest.len()
    }

    #[inline]
    fn get(&self, x: usize, y: usize) -> Option<&u8> {
        self.forest.get(y)?.get(x)
    }
}

impl FromStr for Forest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let forest = s
            .lines()
            .map(|row| {
                row.chars()
                    .map(|tree| tree.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        Ok(Self { forest })
    }
}

#[derive(Debug)]
struct LastSeen {
    vec: Vec<(u8, usize)>,
}

impl LastSeen {
    #[inline]
    const fn new() -> Self {
        Self { vec: Vec::new() }
    }

    #[inline]
    fn see(&mut self, tree: u8, position: usize) {
        if let Some(index) = self.vec.iter().position(|(height, _)| *height == tree) {
            self.vec.remove(index);
        }
        self.vec.push((tree, position));
    }

    #[inline]
    fn iter(&self) -> impl Iterator<Item = &(u8, usize)> {
        self.vec.iter().rev()
    }
}

fn multiply(matrice1: &mut Vec<Vec<usize>>, matrice2: &[Vec<usize>]) {
    for y in 0..matrice1.len() {
        for x in 0..matrice1[0].len() {
            matrice1[y][x] *= matrice2[y][x]
        }
    }
}
