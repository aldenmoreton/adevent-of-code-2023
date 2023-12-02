
fn get_maxs(input: &str) -> (u16, u16, u16) {
    let mut colors = (0, 0, 0);
    let (_, games) = input.split_once(": ").unwrap();
    for game in games.split("; ") {
        for observation in game.split(", ") {
            let (amount, color) = observation.split_once(' ').unwrap();
            let amount = amount.parse().unwrap();
            match color {
                "red" => { if amount > colors.0 { colors.0 = amount} },
                "green" => { if amount > colors.1 { colors.1 = amount} },
                "blue" => { if amount > colors.2 { colors.2 = amount} },
                _ => unreachable!()
            }
        }
    }

    colors
}

fn validate_maxes(observation: (u16, u16, u16), constraint: (u16, u16, u16)) -> bool {
    if observation.0 <= constraint.0 {
        if observation.1 <= constraint.1 {
            observation.2 <= constraint.2
        } else {
            false
        }
    } else {
        false
    }
}

#[aoc(day2, part1)]
fn part_one(input: &str) -> usize {
    let mut id_sum = 0;
    let bag_maxes = (12, 13, 14);
    for (i, line) in input.lines().enumerate() {
        let curr_maxes = get_maxs(line);
        if validate_maxes(curr_maxes, bag_maxes) {
            id_sum += i + 1
        }

    }
    id_sum
}

#[aoc(day2, part2)]
fn part_two(input: &str) -> u16 {
    let mut max_sum_product = 0;
    for line in input.lines() {
        let max_rgb = get_maxs(line);
        max_sum_product += max_rgb.0 * max_rgb.1 * max_rgb.2;
    }
    max_sum_product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1() {
        let input =
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let result = part_one(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn part2_1() {
        let input =
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        let result = part_two(input);
        assert_eq!(result, 2286);
    }

    // #[test]
    // fn part2_1() {
    //     let input = "";
    //     let result = part_two(&input_generator(input));
    //     assert_eq!(result, 0);
    // }
}
