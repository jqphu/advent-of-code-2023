advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let digits = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<Vec<char>>();

        let first_digit = digits[0];

        let second_digit = if digits.len() > 1 {
            digits[digits.len() - 1]
        } else {
            first_digit
        };

        let value = format!("{}{}", first_digit, second_digit)
            .parse::<u32>()
            .unwrap();

        sum += value;
    }

    Some(sum)
}

fn find_digit(input: &str) -> Option<char> {
    if input.contains("one") {
        Some('1')
    } else if input.contains("two") {
        Some('2')
    } else if input.contains("three") {
        Some('3')
    } else if input.contains("four") {
        Some('4')
    } else if input.contains("five") {
        Some('5')
    } else if input.contains("six") {
        Some('6')
    } else if input.contains("seven") {
        Some('7')
    } else if input.contains("eight") {
        Some('8')
    } else if input.contains("nine") {
        Some('9')
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let mut first_digit = None;
        let mut last_digit = None;

        let mut potential_digit = "".to_string();

        // Forward pass looking for digits.
        for c in line.chars() {
            if c.is_ascii_digit() {
                first_digit = Some(c);
                break;
            } else {
                potential_digit += &c.to_string();

                match find_digit(&potential_digit) {
                    Some(digit) => {
                        first_digit = Some(digit);
                        break;
                    }
                    None => {}
                }
            }
        }

        let mut potential_digit = "".to_string();
        // Backwards pass.
        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                last_digit = Some(c);
                break;
            } else {
                // Push to the front
                potential_digit = c.to_string() + &potential_digit;

                match find_digit(&potential_digit) {
                    Some(digit) => {
                        last_digit = Some(digit);
                        break;
                    }
                    None => {}
                }
            }
        }

        let value = format!(
            "{}{}",
            first_digit.unwrap(),
            last_digit.or_else(|| first_digit).unwrap()
        )
        .parse::<u32>()
        .unwrap();

        sum += value;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
