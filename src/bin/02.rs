advent_of_code::solution!(2);

use std::cmp;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0u32;
    'game: for (i, line) in input.lines().enumerate() {
        let pulls = line.split(&[':', ';'][..]).collect::<Vec<&str>>();

        // Check each split
        // Skip first split which is the game number
        for pull in &pulls[1..] {
            let pull = pull.trim();

            for ball in pull.split(",").collect::<Vec<&str>>() {
                let ball = ball.trim();

                let split = ball.split_whitespace().collect::<Vec<&str>>();
                let value = split[0].parse::<u32>().unwrap();
                let color = split[1];

                match color {
                    "red" => {
                        if value > 12 {
                            continue 'game;
                        }
                    }

                    "green" => {
                        if value > 13 {
                            continue 'game;
                        }
                    }
                    "blue" => {
                        if value > 14 {
                            continue 'game;
                        }
                    }
                    _ => panic!("Unexpected value"),
                }
            }
        }

        // Valid game.
        sum += i as u32 + 1;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0u32;
    for line in input.lines() {
        let pulls = line.split(&[':', ';'][..]).collect::<Vec<&str>>();

        let (mut max_red, mut max_blue, mut max_green) = (0, 0, 0);

        // Check each split
        // Skip first split which is the game number
        for pull in &pulls[1..] {
            let pull = pull.trim();

            for ball in pull.split(",").collect::<Vec<&str>>() {
                let ball = ball.trim();

                let split = ball.split_whitespace().collect::<Vec<&str>>();
                let value = split[0].parse::<u32>().unwrap();
                let color = split[1];

                match color {
                    "red" => {
                        max_red = cmp::max(max_red, value);
                    }

                    "green" => {
                        max_green = cmp::max(max_green, value);
                    }
                    "blue" => {
                        max_blue = cmp::max(max_blue, value);
                    }
                    _ => panic!("Unexpected value"),
                }
            }
        }

        // Valid game.
        sum += max_red * max_blue * max_green;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
