use std::str::FromStr;

const WORLD_SIZE: usize = 25;
const OFFSET: usize = 1;
type Coords = (usize, usize, usize);

pub fn part1(input: &str) -> u32 {
    let air = World::from_str(input).unwrap();
    air.count_free_sides()
}

pub fn part2(input: &str) -> u32 {
    let air = World::from_str(input).unwrap();
    air.count_visible_side()
}

struct World {
    grid: [[[bool; WORLD_SIZE]; WORLD_SIZE]; WORLD_SIZE],
    droplets: Vec<Coords>,
}

impl World {
    fn count_free_sides(&self) -> u32 {
        let mut free_sides = 0;
        for &(x, y, z) in self.droplets.iter() {
            if self.grid[x + 1][y][z] {
                free_sides += 2;
            }
            if self.grid[x][y + 1][z] {
                free_sides += 2;
            }
            if self.grid[x][y][z + 1] {
                free_sides += 2;
            }
        }
        self.droplets.len() as u32 * 6 - free_sides
    }

    fn count_visible_side(&self) -> u32 {
        let mut visible_sides = 0;
        let mut seen = [[[false; WORLD_SIZE]; WORLD_SIZE]; WORLD_SIZE];
        let mut to_see = Vec::with_capacity(WORLD_SIZE.pow(2));
        to_see.push((0, 0, 0));

        while let Some((x, y, z)) = to_see.pop() {
            if seen[x][y][z] {
                continue;
            }
            let neighbours = self.get_neighbours_of((x, y, z));
            seen[x][y][z] = true;

            neighbours
                .into_iter()
                .filter(|&(x, y, z)| !seen[x][y][z])
                .for_each(|(x, y, z)| {
                    if self.grid[x][y][z] {
                        visible_sides += 1;
                    } else {
                        to_see.push((x, y, z))
                    }
                })
        }

        visible_sides
    }

    fn get_neighbours_of(&self, (x, y, z): Coords) -> Vec<Coords> {
        vec![
            ((x + 1) % WORLD_SIZE, y, z),
            (if x == 0 { WORLD_SIZE - 1 } else { x - 1 }, y, z),
            (x, (y + 1) % WORLD_SIZE, z),
            (x, if y == 0 { WORLD_SIZE - 1 } else { y - 1 }, z),
            (x, y, (z + 1) % WORLD_SIZE),
            (x, y, if z == 0 { WORLD_SIZE - 1 } else { z - 1 }),
        ]
    }
}

impl FromStr for World {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let droplets = s
            .lines()
            .map(|line| {
                let mut coords = line.split(',').map(|coord| coord.parse::<usize>().unwrap());
                let x = coords.next().unwrap();
                let y = coords.next().unwrap();
                let z = coords.next().unwrap();
                (x + OFFSET, y + OFFSET, z + OFFSET)
            })
            .collect::<Vec<Coords>>();

        let mut grid = [[[false; WORLD_SIZE]; WORLD_SIZE]; WORLD_SIZE];
        for &(x, y, z) in droplets.iter() {
            grid[x][y][z] = true;
        }

        Ok(Self { grid, droplets })
    }
}
