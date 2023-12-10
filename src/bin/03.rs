advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    // Even though the input is 140x140 we add an extra layer of '.' around everything to make it
    // easier to parse. This guarnatees no number exists without being surrounded by .
    let mut input_2d = [['.'; 142]; 142];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                input_2d[i + 1][j + 1] = c;
            }
        }
    }

    let mut i = 0;

    let mut total = 0;

    // 1. Line by line look for numbers. (note we assume number do NOT wrap because we checked the
    //    input).
    //
    // Once we find the start of a number - keep track of indexs.
    while i < 142 {
        // We don't need to track j since we will never increment i when looking for a number.
        let mut j = 0;
        let mut number_j_start = 0;
        let mut number = "".to_string();

        while j < 142 {
            // Found start of character.
            //
            // We always add this character. This could be the start of a sequence.
            if input_2d[i][j].is_ascii_digit() {
                // Start of a new sequence.
                if number == "" {
                    number_j_start = j;
                }

                number += &input_2d[i][j].to_string();
            }

            // println!("[{i}][{j}]: [{}], number: {number}", input_2d[i][j]);

            // Look ahead one to see if we finished the number.
            //
            // SAFETY: this is safe as there is never a number at the edge based on the input.
            if number != "" && !input_2d[i][j + 1].is_ascii_digit() {
                let number_value = number.parse::<u32>().unwrap();
                // println!("Evaluating: {number_value}");

                // Look around for each index to determine if there's an issue.

                // Look left
                if is_symbol(input_2d[i][number_j_start - 1]) ||
                // Look right
                is_symbol(input_2d[i][j + 1]) ||
                // Diag: Top Left
                is_symbol(input_2d[i-1][number_j_start - 1]) ||
                // Diag: Bottom Left
                is_symbol(input_2d[i+1][number_j_start - 1]) ||
                // Diag: Top Right
                is_symbol(input_2d[i-1][j + 1]) ||
                // Diag: Bottom Right
                is_symbol(input_2d[i+1][j + 1])
                {
                    // println!("Adding number {number_value}");
                    total += number_value;
                } else {
                    // Look up and down.
                    //
                    // number_index is the index of each number.
                    for number_index in number_j_start..j + 1 {
                        // Look up
                        if is_symbol(input_2d[i+1][number_index]) ||
                    // Look down
                    is_symbol(input_2d[i-1][number_index])
                        {
                            // println!("Adding number {number_value}");
                            // Found a valid number, let's move to next.
                            total += number_value;
                        }
                    }
                }

                // Reset the number.
                number_j_start = 0;
                number = "".to_string();
            }

            j += 1;
        }

        i += 1;
    }

    Some(total)
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

pub fn part_two(input: &str) -> Option<u32> {
    // Even though the input is 140x140 we add an extra layer of '.' around everything to make it
    // easier to parse. This guarnatees no number exists without being surrounded by .
    let mut input_2d = [['.'; 142]; 142];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                input_2d[i + 1][j + 1] = c;
            }
        }
    }

    let mut total = 0;

    // If we've checked this for a digit.
    let mut seen = [[false; 142]; 142];

    // Algorithm:
    //
    // Find each "*" and check around for a number.
    //
    // NOTE: Watch out for a number spanning areas we want to check. Simply mark each section
    // around as "seen" if it's hit by a number.

    for  i in 0..142 {
        for  j in 0..142 {

            // It's an asterick. Check around.
            if input_2d[i][j] == '*' {

                // println!("* found at {i},{j}");

                let mut numbers: Vec<u32> = vec![];

                // Let's just look in a 3x3 for ease. We will never find a number at the asterick
                // anyway.
                for box_i in -1i32..=1 {
                    for box_j in -1i32..=1 {
                        if let Some(number_found) = find_number(&mut seen, &input_2d, (i as i32 + box_i) as usize, (j as i32 + box_j) as usize) {
                            // println!("Found number {number_found}");
                            numbers.push(number_found);
                        }
                    }
                }

                if numbers.len() == 2 {
                    let value: u32 = numbers.iter().fold(1, |acc, &num| acc * num);

                    // println!("Value: {value}");
                    total += value;
                }
            }
        }
    }

    Some(total)
}

// Look at index i,j to see if there is a number.
pub fn find_number(seen: &mut [[bool; 142]; 142], input: &[[char; 142];142], i: usize, j: usize) -> Option<u32> {


    // println!("Looking for number at {i},{j}");

    if !input[i][j].is_ascii_digit() {
        // println!("Not ascii {i},{j}");
        return None;
    }

    // Already used this index in a number.
    if seen[i][j] {
        // println!("Seen {i},{j}");
        return None;
    }

    // println!("Parsing number at {i},{j}");

    // It's a digit we haven't seen. Start going left and right to generate a number.

    let mut number_start_j = j;
    let mut number_end_j = j;

    // Put number to the left to find the first character.
    while input[i][number_start_j - 1].is_ascii_digit() {
        number_start_j -= 1;
    }

    // Put number to the right
    //
    // Get index of last character plus 1.
    while input[i][number_end_j].is_ascii_digit() {
        number_end_j += 1;
    }

    // Extract the number
    let value = &input[i][number_start_j..number_end_j].iter().collect::<String>().parse::<u32>().unwrap();

    // Mark all these values as seen.
    for seen_j in number_start_j..number_end_j {
        seen[i][seen_j] = true;
    }

    Some(*value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
