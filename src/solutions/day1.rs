pub fn part1(input: &str) -> u32 {
    input
        .split("\r\n\r\n")
        .map(|elve| elve.lines().map(|food| food.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> u32 {
    let food_stack = input
        .split("\r\n\r\n")
        .map(|elve| elve.lines().map(|food| food.parse::<u32>().unwrap()).sum())
        .collect::<Vec<_>>();

    let mut max3 = (food_stack[0], food_stack[1], food_stack[2]);
    sort3(&mut max3);

    for calorie in food_stack.into_iter().skip(3) {
        if max3.0 < calorie {
            max3.0 = calorie;
            sort3(&mut max3)
        }
    }

    max3.0 + max3.1 + max3.2
}

fn sort3(triplet: &mut (u32, u32, u32)) {
    *triplet = if triplet.0 <= triplet.1 {
        if triplet.1 <= triplet.2 {
            (triplet.0, triplet.1, triplet.2)
        } else if triplet.0 <= triplet.2 {
            (triplet.0, triplet.2, triplet.1)
        } else {
            (triplet.2, triplet.0, triplet.1)
        }
    } else if triplet.0 <= triplet.2 {
        (triplet.1, triplet.0, triplet.2)
    } else if triplet.1 <= triplet.2 {
        (triplet.1, triplet.2, triplet.0)
    } else {
        (triplet.2, triplet.1, triplet.0)
    }
}
