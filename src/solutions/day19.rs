use std::str::FromStr;

use derive_more::{AddAssign, SubAssign};

const MAX_TIME: u32 = 10;

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let blueprint = BluePrint::from_str(line).unwrap();
            dbg!(blueprint.id) * dbg!(blueprint.simulat().0)
        })
        .sum()
    // .collect::<Vec<_>>();
}

pub fn part2(_input: &str) -> u32 {
    0
}

enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, AddAssign, SubAssign)]
struct Ore(u32);
#[derive(Clone, Copy, PartialEq, PartialOrd, AddAssign, SubAssign)]
struct Clay(u32);
#[derive(Clone, Copy, PartialEq, PartialOrd, AddAssign, SubAssign)]
struct Obsidian(u32);
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, AddAssign)]
struct Geode(u32);

struct BluePrint {
    id: u32,
    ore_robot_cost: Ore,
    clay_robot_cost: Ore,
    obsidian_robot_cost: (Ore, Clay),
    geode_robot_cost: (Ore, Obsidian),
}

impl BluePrint {
    fn simulat(&self) -> Geode {
        self.simulat_inner(Storage::new(), Fleet::new(), 0)
    }

    fn simulat_inner(&self, storage: Storage, fleet: Fleet, time: u32) -> Geode {
        println!("Time! {time}");
        if time == MAX_TIME {
            return storage.geode;
        }

        let no_buy_choice = self.simulat_inner(fleet.produce(&storage), fleet.clone(), time + 1);

        [
            Material::Ore,
            Material::Clay,
            Material::Obsidian,
            Material::Ore,
            Material::Geode,
        ]
        .into_iter()
        .filter_map(|material| {
            fleet
                .buy_robot(self, &storage, material)
                .map(|(new_fleet, storage)| {
                    self.simulat_inner(fleet.produce(&storage), new_fleet, time + 1)
                })
        })
        .chain(std::iter::once(no_buy_choice))
        .max()
        .unwrap()
    }
}

impl FromStr for BluePrint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s[10..s.len() - 10];

        let (id, s) = s.split_once(':').unwrap();

        let ore_robot_cost = Ore(s[22..23].parse::<u32>().unwrap());
        let clay_robot_cost = Ore(s[51..52].parse::<u32>().unwrap());

        let obsidian_robot_ore_cost = Ore(s[84..85].parse::<u32>().unwrap());
        let s = &s[94..];
        let (obsidian_robot_clay_cost, s) = s.split_once(' ').unwrap();

        let geode_robot_ore_cost = Ore(s[29..30].parse::<u32>().unwrap());
        let geode_robot_obsidian_cost = Obsidian(s[s.len() - 2..].trim().parse::<u32>().unwrap());

        Ok(Self {
            id: id.parse().unwrap(),
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost: (
                obsidian_robot_ore_cost,
                Clay(obsidian_robot_clay_cost.parse().unwrap()),
            ),
            geode_robot_cost: (geode_robot_ore_cost, geode_robot_obsidian_cost),
        })
    }
}

#[derive(Clone)]
struct Storage {
    ore: Ore,
    clay: Clay,
    obsidian: Obsidian,
    geode: Geode,
}

impl Storage {
    fn new() -> Self {
        Self {
            ore: Ore(0),
            clay: Clay(0),
            obsidian: Obsidian(0),
            geode: Geode(0),
        }
    }
}

#[derive(Clone)]
struct Fleet {
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

impl Fleet {
    fn new() -> Self {
        Self {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }

    fn produce(&self, storage: &Storage) -> Storage {
        let mut storage = storage.clone();
        storage.ore += Ore(self.ore_robots);
        storage.clay += Clay(self.clay_robots);
        storage.obsidian += Obsidian(self.obsidian_robots);
        storage.geode += Geode(self.geode_robots);
        storage
    }

    fn buy_robot(
        &self,
        blueprint: &BluePrint,
        storage: &Storage,
        material: Material,
    ) -> Option<(Fleet, Storage)> {
        let mut storage = storage.clone();
        let mut fleet = self.clone();
        match material {
            Material::Ore if blueprint.ore_robot_cost <= storage.ore => {
                storage.ore -= blueprint.ore_robot_cost;
                fleet.ore_robots += 1;
            }
            Material::Clay if blueprint.clay_robot_cost <= storage.ore => {
                storage.ore -= blueprint.clay_robot_cost;
                fleet.clay_robots += 1;
            }
            Material::Obsidian
                if blueprint.obsidian_robot_cost.0 <= storage.ore
                    && blueprint.obsidian_robot_cost.1 <= storage.clay =>
            {
                storage.ore -= blueprint.obsidian_robot_cost.0;
                storage.clay -= blueprint.obsidian_robot_cost.1;
                fleet.obsidian_robots += 1;
            }
            Material::Geode
                if blueprint.geode_robot_cost.0 <= storage.ore
                    && blueprint.geode_robot_cost.1 <= storage.obsidian =>
            {
                storage.ore -= blueprint.geode_robot_cost.0;
                storage.obsidian -= blueprint.geode_robot_cost.1;
                fleet.geode_robots += 1;
            }
            _ => return None,
        }

        Some((fleet, storage))
    }
}
