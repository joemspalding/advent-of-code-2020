mod constants;

pub mod day_one {
    const EXPENSE_REPORT: [usize; 200] = super::constants::DAY_ONE;

    pub fn find_2020_product_two_num() -> Option<usize> {
        for i in 0..EXPENSE_REPORT.len() {
            for j in i..EXPENSE_REPORT.len() {
                if EXPENSE_REPORT[i] + EXPENSE_REPORT[j] == 2020 {
                     return Some(EXPENSE_REPORT[i] * EXPENSE_REPORT[j]);
                }
            }
        }

        None
    }

    pub fn find_2020_product_three_num() -> Option<usize>  {
        for i in 0..EXPENSE_REPORT.len() {
            for j in i..EXPENSE_REPORT.len() {
                for k in j..EXPENSE_REPORT.len() {
                    if EXPENSE_REPORT[i] + EXPENSE_REPORT[j] + EXPENSE_REPORT[k] == 2020 {
                        return Some(EXPENSE_REPORT[i] * EXPENSE_REPORT[j] * EXPENSE_REPORT[k]);
                   }
                }
            }
        }

        None
    }

}

pub mod day_two {
    const CORRUPTED_PASSWORDS: [(usize, usize, char, &str); 1000] = super::constants::DAY_TWO;

    struct PasswordFormat {
        min_count: usize,
        max_count: usize,
        letter: char,
        password: String
    }

    impl PasswordFormat {
        pub fn new(
            min_count: usize,
            max_count: usize,
            letter: char,
            password: String
        ) -> Self {
            Self {
                min_count,
                max_count,
                letter,
                password,
            }
        }

        pub fn is_valid(&self) -> bool {
            let count = self.password.chars().filter(|c| *c == self.letter).count();
            count >= self.min_count && count <= self.max_count
        }

        pub fn is_valider(&self) -> bool {
            let positions: (Option<bool>, Option<bool>) = self.password.chars()
            .enumerate()
            .fold((None, None), |mut acc, (i, x)| {
                let index = i+1;
                if index == self.min_count {
                    acc.0 = Some(x == self.letter);
                } else if index == self.max_count {
                    acc.1 = Some(x == self.letter);
                }
                acc
            });

            let is_first_letter_match = positions.0.unwrap();
            let is_second_letter_match = positions.1.unwrap();

            is_first_letter_match && !is_second_letter_match
            || !is_first_letter_match && is_second_letter_match
        }
    }


    pub fn find_valid_passwords() -> usize {
        CORRUPTED_PASSWORDS.iter()
            .map(|x| PasswordFormat::new(x.0, x.1, x.2, x.3.to_string()))
            .filter(|pw| pw.is_valid())
            .count()
    }

    pub fn find_valider_passwords() -> usize {
        CORRUPTED_PASSWORDS.iter()
            .map(|x| PasswordFormat::new(x.0, x.1, x.2, x.3.to_string()))
            .filter(|pw| pw.is_valider())
            .count()
    }
}

pub mod day_three {
    pub const SLOPE_PATTERN: &str = super::constants::DAY_THREE;
    #[derive(Debug, Clone)]
    struct SlopePattern {
        slope_pattern: Vec<String>, // line length is 31
        line_length: usize
    }

    impl SlopePattern {
        pub fn new() -> Self {
            let slope_pattern: Vec<String> = SLOPE_PATTERN.lines()
                .map(|s| s.to_string())
                .collect();

            let line_length = slope_pattern[0].chars().count();
            Self {
                slope_pattern,
                line_length 
            }
        }

        pub fn get_slope_patterns(self) -> Vec<String> {
            self.slope_pattern
        }
        pub fn get_line_legth(self) -> usize {
            self.line_length
        }
    }


    pub fn find_trees() -> usize {
        let slope_pattern = SlopePattern::new();
        let slope_patterns = slope_pattern.clone().get_slope_patterns();
        let line_length = slope_pattern.clone().get_line_legth();


        let mut trees = 0;
        let mut horizontal_position: usize = 0;

        for i in 0..slope_patterns.iter().count() {
            trees = {
                if slope_patterns[i].as_bytes()[horizontal_position] as char == '#' {
                    trees + 1
                } else {
                    trees
                }
            };

            // update the position in the char array
            horizontal_position += 3;
            if horizontal_position >= line_length {
                horizontal_position = horizontal_position - line_length;
            }

        }

        trees
    }
    #[derive(Debug, Clone, Copy)]
    struct TreeCountingObj {
        horizontal_increment: usize,
        vertical_increment: usize,
        horizontal_position: usize,
        trees: usize
    }

    pub fn find_trees_all_patterns() -> usize {
        let slope_pattern = SlopePattern::new();
        let slope_patterns = slope_pattern.clone().get_slope_patterns();
        let line_length = slope_pattern.clone().get_line_legth();

        let mut tree_counting_obj_array = vec![
            TreeCountingObj {horizontal_increment: 1, vertical_increment: 1, horizontal_position: 0, trees: 0},
            TreeCountingObj {horizontal_increment: 3, vertical_increment: 1, horizontal_position: 0, trees: 0},
            TreeCountingObj {horizontal_increment: 5, vertical_increment: 1, horizontal_position: 0, trees: 0},
            TreeCountingObj {horizontal_increment: 7, vertical_increment: 1, horizontal_position: 0, trees: 0},
            TreeCountingObj {horizontal_increment: 1, vertical_increment: 2, horizontal_position: 0, trees: 0},
        ];

        for i in 0..slope_patterns.iter().count() {
            tree_counting_obj_array = tree_counting_obj_array.iter()
            .map(|x| *x)
            .map(|mut x| {
                // go through array and figure out if vertical increment matches current line number using "%"
                if i == 0 || i % x.vertical_increment == 0 {

                    // if it does, then check the horizontal increment for "#", if it matches, increment the trees and update the horizontal position at that element
                    x.trees = {
                        if slope_patterns[i].as_bytes()[x.horizontal_position] as char == '#' {
                            x.trees + 1
                        } else {
                            x.trees
                        }
                    };

                    x.horizontal_position += x.horizontal_increment;
                    if x.horizontal_position >= line_length {
                        x.horizontal_position = x.horizontal_position - line_length;
                    }
                }
                x
            }).collect();
        }
        tree_counting_obj_array.iter().fold(1 as usize, |acc, current| {acc * current.trees})

    }
}
