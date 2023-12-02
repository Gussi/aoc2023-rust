pub mod part1 {

    pub fn main() {
        let input = crate::read_input();

        let mut sum = 0;

        for line in input.lines() {
            let mut two_chars = String::new();

            for c in line.chars() {
                if c.is_digit(10) {
                    two_chars.push(c);
                    break;
                }
            }

            for c in line.chars().rev() {
                if c.is_digit(10) {
                    two_chars.push(c);
                    break;
                }
            }

            let num: i32 = two_chars.parse().unwrap();

            sum += num;
        }

        let answer = sum;

        // print the sum (our answer)
        println!("Answer: {}", answer);
    }
}

pub mod part2 {

    pub fn main() {
        let input = crate::read_input();

        let number_words = vec![
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let mut sum = 0;

        for line in input.lines() {
            let mut two_chars = String::new();

            'outer: for (i, c) in line.chars().enumerate() {
                if c.is_digit(10) {
                    two_chars.push(c);
                    break;
                }

                if c.is_alphabetic() {
                    // Check if number word is found in line
                    for (j, number_word) in number_words.iter().enumerate() {
                        if line[i..].starts_with(number_word) {
                            // Convert number word to digit
                            two_chars.push_str(&j.to_string());
                            break 'outer;
                        }
                    }
                }
            }

            // Loop backawards through line
            'outer: for (i, c) in line.chars().rev().enumerate() {
                // Get index of character in line
                let j = line.len() - i - 1;

                if c.is_digit(10) {
                    two_chars.push(c);
                    break;
                }

                if c.is_alphabetic() {
                    // Check if number word is found in line
                    for (k, number_word) in number_words.iter().enumerate() {
                        if line[j..].starts_with(number_word) {
                            // Convert number word to digit
                            two_chars.push_str(&k.to_string());
                            break 'outer;
                        }
                    }
                }
            }

            let num: i32 = two_chars.parse().unwrap();

            sum += num;
        }

        println!("Answer: {}", sum);
    }
}
