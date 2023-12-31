use std::collections::VecDeque;
use itertools::Itertools;

use rayon::prelude::*;

type IDs = Vec<u64>;
type Mapping = ((u64, u64), u64);
type Mappings = Vec<Vec<Mapping>>;

#[aoc_generator(day5)]
fn input_generator(input: &str) -> (IDs, Mappings) {
    let mut sections = input.split("\n\n");

    let seeds = {
        let (_, seeds) = sections.next().unwrap().split_once(": ").unwrap();
        seeds
            .split_whitespace()
            .map(|number| number.parse::<u64>().unwrap())
            .collect_vec()
    };

    let value_mappings = sections
        .map(|mapping|
            mapping
            .lines()
            .skip(1)
            .map(|line| {
                let mut amounts = line.split_whitespace();
                (
                    (
                        amounts.next().unwrap().parse::<u64>().unwrap(),
                        amounts.next().unwrap().parse::<u64>().unwrap()
                    ),
                    amounts.next().unwrap().parse::<u64>().unwrap()
                )
            })
            .collect_vec()
        )
        .collect_vec();

    (seeds, value_mappings)
}

#[aoc(day5, part1)]
fn part_one(input: &(IDs, Mappings)) -> u64 {
    let mut ids = input.0.clone();
    let mappings = &input.1;

    let mut min_id = u64::MAX;
    for id in ids.iter_mut() {
        // print!("{} -> ", id);
        for mapping in mappings.iter() {
            for ((value_start, key_start), map_len) in mapping {
                if key_start <= id && *id < key_start + map_len {
                    // print!("*");
                    let key_offset = *id - key_start;
                    *id = value_start + key_offset;
                    break
                }
            }
            // print!("{} -> ", id);
        }
        // println!("{}", id);
        if *id < min_id { min_id = *id }
    }

    min_id
}


#[aoc(day5, part1, Rayon)]
fn part_one_rayon(input: &(IDs, Mappings)) -> u64 {
    let ids = input.0.clone();
    let mappings = &input.1;

    ids
        .into_par_iter()
        .map(|mut id| {
            for mapping in mappings.iter() {
                for ((value_start, key_start), map_len) in mapping {
                    if *key_start <= id && id < key_start + map_len {
                        let key_offset = id - key_start;
                        id = value_start + key_offset;
                        break
                    }
                }
            }
            id
        })
        .min()
        .unwrap()
}

type NextRange = Option<(u64, u64)>;
type Leftover = Option<(u64, u64)>;
fn split_ranges(
    (curr_start, curr_len): (u64, u64),
    ((value_start, key_start), mapping_len): Mapping
) -> (NextRange, Leftover, Leftover) {
    let curr_end = curr_start + curr_len - 1;
    let key_end = key_start + mapping_len - 1;
    match curr_start.cmp(&key_start) {
        std::cmp::Ordering::Less => {
            match curr_end.cmp(&key_end) {
                std::cmp::Ordering::Less => {
                    if curr_end < key_start {
                        // println!("No overlap");
                        (None, Some((curr_start, curr_len)), None)
                    } else {
                        // Case 1
                        // println!("Case 1: {}-{}", curr_start, curr_end);
                        let left_len = key_start - curr_start;
                        (
                            Some((value_start, 1 + curr_end - key_start)),
                            Some((curr_start, left_len)),
                            None
                        )
                    }
                },
                std::cmp::Ordering::Greater => {
                    // Case 2
                    // println!("Case 2: {}-{}", curr_start, curr_end);
                    let left_len = key_start - curr_start;
                    let right_len = curr_end - key_end;
                    (
                        Some((value_start, mapping_len)),
                        Some((curr_start, left_len)),
                        Some((key_end+1, right_len))
                    )
                },
                std::cmp::Ordering::Equal => {
                    // Case 3
                    // println!("Case 3: {}-{}", curr_start, curr_end);
                    let left_len = key_start - curr_start;
                    (
                        Some((value_start, mapping_len)),
                        Some((curr_start, left_len)),
                        None
                    )
                }
            }
        },
        std::cmp::Ordering::Greater => {
            match curr_end.cmp(&key_end) {
                std::cmp::Ordering::Less => {
                    // Case 4
                    // println!("Case 4: {}-{}", curr_start, curr_end);
                    (
                        Some((value_start + (curr_start - key_start), curr_len)),
                        None,
                        None
                    )
                },
                std::cmp::Ordering::Greater => {
                    if curr_start > key_end {
                        // println!("No overlap");
                        return (None, Some((curr_start, curr_len)), None)
                    }
                    // Case 5
                    // println!("Case 5: {}-{}", curr_start, curr_end);
                    let right_len = curr_end - key_end;
                    (
                        Some((value_start + (curr_start - key_start), curr_len - right_len)),
                        Some((key_end + 1, right_len)),
                        None
                    )
                },
                std::cmp::Ordering::Equal => {
                    // Case 6
                    // println!("Case 6: {}-{}", curr_start, curr_end);
                    (
                        Some((value_start + (curr_start - key_start), curr_len)),
                        None,
                        None
                    )
                }
            }
        },
        std::cmp::Ordering::Equal => {
            match curr_end.cmp(&key_end) {
                std::cmp::Ordering::Less => {
                    // Case 7
                    // println!("Case 7: {}-{}", curr_start, curr_end);
                    (
                        Some((value_start, curr_len)),
                        None,
                        None
                    )
                },
                std::cmp::Ordering::Greater => {
                    // Case 8
                    // println!("Case 8: {}-{}", curr_start, curr_end);
                    let right_len = curr_end - key_end;
                    (
                        Some((value_start, curr_len - right_len)),
                        Some((key_end + 1, right_len)),
                        None
                    )
                },
                std::cmp::Ordering::Equal => {
                    // Case 9
                    // println!("Case 9: {}-{}", curr_start, curr_end);
                    (
                        Some((value_start, mapping_len)),
                        None,
                        None
                    )
                }
            }
        }
    }
}

#[aoc(day5, part2)]
fn part_two_optimized((ids, mappings): &(IDs, Mappings)) -> u64 {
    let id_ranges: Vec<(u64, u64)> = ids
        .clone()
        .into_iter()
        .tuples()
        .collect();

    let mut total_min = u64::MAX;
    for id_range in id_ranges {
        let mut curr_ranges = VecDeque::new();
        curr_ranges.push_back(id_range);

        for mapping in mappings {
            let mut next_ranges = VecDeque::new();
            'range_splitting: while let Some(mut curr_range) = curr_ranges.pop_front() {
                for map in mapping {
                    let (next_range, new_curr, new_leftover) = split_ranges(curr_range, *map);
                    if let Some(range) = next_range {
                        next_ranges.push_back(range)
                    }
                    if let Some(leftover) = new_curr {
                        curr_range = leftover
                    } else {
                        continue 'range_splitting
                    }
                    if let Some(leftover) = new_leftover {
                        curr_ranges.push_back(leftover)
                    }
                }
                next_ranges.push_back(curr_range)
            }
            curr_ranges = next_ranges;
        }

        let curr_min = curr_ranges
            .into_iter()
            .map(|(start_value, _)| start_value)
            .min()
            .unwrap();
        if curr_min < total_min { total_min = curr_min }
    }

    total_min
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_1() {
        let input = indoc!{"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "};
        let result = part_one(&input_generator(input));
        assert_eq!(result, 35);
    }


    #[test]
    fn part2_1() {
        let input = indoc!{"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "};
        let result = part_two_optimized(&input_generator(input));
        assert_eq!(result, 46);
    }
}
