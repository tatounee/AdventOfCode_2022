use std::str::FromStr;

type Coords = (i64, i64);

#[allow(non_snake_case)]
pub fn part1(input: &str) -> i64 {
    let sensors = input
        .lines()
        .map(|line| Sensor::from_str(line).unwrap())
        .collect::<Vec<_>>();

    let within_reach_of_2000000 = scanned_ranges(&sensors, 2_000_000);

    within_reach_of_2000000
        .iter()
        .map(|(start, end)| end - start)
        .sum::<i64>()
}

pub fn part2(input: &str) -> i64 {
    const MAX_Y: i64 = 4_000_000;

    let sensors = input
        .lines()
        .map(|line| Sensor::from_str(line).unwrap())
        .collect::<Vec<_>>();

    // let ranges = scanned_ranges(&sensors, 11);

    for y in 0..MAX_Y {
        let ranges = scanned_ranges(&sensors, y);
        if ranges.len() == 1 {
            continue;
        }

        return (ranges[0].1 + 1) * MAX_Y + y;
    }

    unreachable!()
}

fn scanned_ranges(sensors: &[Sensor], target_y: i64) -> Vec<(i64, i64)> {
    let mut within_reach_of_target = sensors
        .iter()
        .filter_map(|sensor| {
            let distance_of_target = target_y.abs_diff(sensor.position.1);
            if distance_of_target < sensor.distance {
                Some((
                    sensor.position.0 - (sensor.distance - distance_of_target) as i64,
                    sensor.position.0 + (sensor.distance - distance_of_target) as i64,
                ))
            } else {
                None
            }
        })
        .collect::<Vec<Coords>>();
    within_reach_of_target.sort_unstable_by_key(|(start, _)| *start);

    // for (a, b) in within_reach_of_target.iter() {
    //     println!("{a} - {b} : {}", b - a);
    // }

    let mut to_remove = Vec::new();
    for i in 0..(within_reach_of_target.len() - 1) {
        if within_reach_of_target[i].1 >= within_reach_of_target[i + 1].1 {
            within_reach_of_target[i + 1] = within_reach_of_target[i];
            to_remove.push(i)
        } else if within_reach_of_target[i].1 >= within_reach_of_target[i + 1].0 {
            within_reach_of_target[i + 1].0 = within_reach_of_target[i].0;
            to_remove.push(i)
        }
    }
    for idx in to_remove.into_iter().rev() {
        within_reach_of_target.swap_remove(idx);
    }

    within_reach_of_target
}

struct Sensor {
    position: Coords,
    beacon: Coords,
    distance: u64,
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor, beacon) = s.split_once(':').ok_or(())?;

        let (sensor_x, sensor_y) = sensor[12..].split_once(", y=").ok_or(())?;
        let sensor_x = sensor_x.parse::<i64>().map_err(|_| ())?;
        let sensor_y = sensor_y.parse::<i64>().map_err(|_| ())?;

        let (beacon_x, beacon_y) = beacon[24..].split_once(", y=").ok_or(())?;
        let beacon_x = beacon_x.parse::<i64>().map_err(|_| ())?;
        let beacon_y = beacon_y.parse::<i64>().map_err(|_| ())?;

        Ok(Self::new((sensor_x, sensor_y), (beacon_x, beacon_y)))
    }
}

impl Sensor {
    fn new(position: Coords, beacon: Coords) -> Self {
        Self {
            position,
            beacon,
            distance: manhattan_distance(position, beacon),
        }
    }
}

fn manhattan_distance((x1, y1): Coords, (x2, y2): Coords) -> u64 {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}
