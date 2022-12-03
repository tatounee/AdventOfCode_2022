pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|rucksack| {
            let compartment_1 = items_to_u128(&rucksack[..rucksack.len() / 2]);
            let compartment_2 = items_to_u128(&rucksack[(rucksack.len() / 2)..]);

            let shared = compartment_1 & compartment_2;
            shared.trailing_zeros() + 1
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .array_chunks::<3>()
        .map(|[bag1, bag2, bag3]| {
            (items_to_u128(bag1) & items_to_u128(bag2) & items_to_u128(bag3)).trailing_zeros() + 1
        })
        .sum()
}

fn items_to_u128(string: &str) -> u128 {
    let mut output = 0u128;

    for item in string.bytes() {
        if item >= 97 {
            // lower letters
            output |= 1 << (item - 97)
        } else {
            // UPPER LETTERS
            output |= 1 << (item - 65 + 26)
        }
    }

    output
}

#[test]
fn convertion_str_to_u128() {
    let s = "ZaczA";
    let u = items_to_u128(s);
    assert_eq!(u, 0b1000000000000000000000000110000000000000000000000101)
}
