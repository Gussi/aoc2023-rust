pub mod part1 {
    pub fn solve() -> String {
        let input = crate::read_input();
        let mut sum_of_calibration_values = 0;

        for line in input.lines() {
            let mut calibration_value = String::new();

            for c in line.chars() {
                if c.is_digit(10) {
                    calibration_value.push(c);
                    break;
                }
            }

            for c in line.chars().rev() {
                if c.is_digit(10) {
                    calibration_value.push(c);
                    break;
                }
            }

            let num: i32 = calibration_value.parse().unwrap();

            sum_of_calibration_values += num;
        }

        return format!("Sum of all calibration values: {}", sum_of_calibration_values);
    }
}

pub mod part2 {
    pub fn solve() -> String {
        let input = crate::read_input();

        let number_words = vec![
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let mut sum_of_calibration_values = 0;

        for line in input.lines() {
            let mut calibration_value = String::new();

            'outer: for (i, c) in line.chars().enumerate() {
                if c.is_digit(10) {
                    calibration_value.push(c);
                    break;
                }

                if c.is_alphabetic() {
                    for (j, number_word) in number_words.iter().enumerate() {
                        if line[i..].starts_with(number_word) {
                            calibration_value.push_str(&j.to_string());
                            break 'outer;
                        }
                    }
                }
            }

            'outer: for (i, c) in line.chars().rev().enumerate() {
                let j = line.len() - i - 1;

                if c.is_digit(10) {
                    calibration_value.push(c);
                    break;
                }

                if c.is_alphabetic() {
                    for (k, number_word) in number_words.iter().enumerate() {
                        if line[j..].starts_with(number_word) {
                            calibration_value.push_str(&k.to_string());
                            break 'outer;
                        }
                    }
                }
            }

            let num: i32 = calibration_value.parse().unwrap();

            sum_of_calibration_values += num;
        }

        return format!("Sum of all calibration values: {}", sum_of_calibration_values);
    }
}
