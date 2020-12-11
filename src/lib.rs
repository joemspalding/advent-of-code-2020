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

pub mod day_four {
    const PASSPORT_DATA: &str = super::constants::DAY_FOUR;
    #[derive(Debug, Clone)]
    struct PassportData {
        birth_year: Option<String>, // byr
        issue_year: Option<String>, // iyr
        expiration_year: Option<String>, // eyr
        height: Option<String>, // hgt
        hair_color: Option<String>, // hcl
        eye_color: Option<String>, // ecl
        passport_id: Option<String>, // pid
        country_id: Option<String>, // cid
    }

    impl PassportData {
        fn new(input: &str) -> Self {
            // string data is disorganized and may omit fields
            input.split(|c| c == ' ' || c == '\u{000A}')
            .fold(PassportData::new_empty(), |mut acc: PassportData, current: &str| {
                match &current[0..3] {
                    "byr" => {
                        acc.birth_year = Some(current[4..current.len()].to_string());
                        acc
                    },
                    "iyr" => {
                        acc.issue_year = Some(current[4..current.len()].to_string());
                        acc
                    },
                    "eyr" => {
                        acc.expiration_year = Some(current[4..current.len()].to_string());
                        acc
                    },
                    "hgt" => {
                        acc.height = Some(current[4..current.len()].to_string());
                        acc
                    },
                    "hcl" => {
                        acc.hair_color = Some(current[4..current.len()].to_string());
                        acc
                    },
                    "ecl" => {
                        acc.eye_color = Some(current[4..current.len()].to_string());
                        acc
                    },
                    "pid" => {
                        acc.passport_id = Some(current[4..current.len()].to_string());
                        acc
                    },
                    "cid" => {
                        acc.country_id = Some(current[4..current.len()].to_string());
                        acc
                    },
                    _ => acc,
                }
            })
        }
        fn new_empty() -> Self {
            Self {
                birth_year: None, // byr
                issue_year: None, // iyr
                expiration_year: None, // eyr
                height: None, // hgt
                hair_color: None, // hcl
                eye_color: None, // ecl
                passport_id: None, // pid
                country_id: None, // cid
            }
        }

        fn has_required_fields(&self) -> bool {  
            self.birth_year != None
            && self.issue_year != None
            && self.expiration_year != None
            && self.height != None
            && self.hair_color != None
            && self.eye_color != None
            && self.passport_id != None
        }

        fn has_fields_with_valid_data(self) -> Result<bool, Box<dyn std::error::Error>> {
            
            if !self.has_required_fields() {
                Ok(false)
            } else {
                let birth_year: usize = match self.birth_year {
                    Some(n) => Ok(n.parse::<usize>()?),
                    None => Err(NoneError)
                }?;
                let issue_year: usize = match self.issue_year {
                    Some(n) => Ok(n.parse::<usize>()?),
                    None => Err(NoneError)
                }?;
                let expiration_year: usize = match self.expiration_year {
                    Some(n) => Ok(n.parse::<usize>()?),
                    None => Err(NoneError)
                }?;

                let is_valid = Ok(
                    1920 <= birth_year && birth_year <= 2002 // solving for birth year
                    && 2010 <= issue_year && issue_year <= 2020 // solving for issue year
                    && 2020 <= expiration_year && expiration_year <= 2030 // solving for expiration year
                    && { // Solving for height
                        match self.height {
                            Some(n) => {
                                let measurement = &n[n.len()-2..n.len()]; // expecting "cm" or "in"
                                match measurement {
                                    "cm" => {
                                        let cm: usize = n[..3].parse()?;
                                        Ok(150 <= cm && cm <= 193)
                                    },
                                    "in" => {
                                        let inch: usize = n[..2].parse()?;
                                        Ok(59 <= inch && inch <= 76)
                                    },
                                    _ => Ok(false)
                                }
                            },
                            None => Err(NoneError)
                        }
                    }? && { // solving for hair color (#0-9, a-f)
                        match self.hair_color {
                            Some(n) => {
                                let regex = regex::Regex::new(r"^#(\d|[a-f]){6}$").unwrap();
                                let ismatch = regex.is_match(&n.to_string());
                                Ok(ismatch)
                            },
                            None => Err(NoneError)
                        }
                    }? && { // solving for eye color
                        match self.eye_color {
                            Some(n) => {
                                Ok(n == "amb"
                                || n == "blu"
                                || n == "brn"
                                || n == "gry"
                                || n == "grn"
                                || n == "hzl"
                                || n == "oth")
                            },
                            None => Err(NoneError)
                        }
                    }? && { // solve for passport id
                        match self.passport_id {
                            Some(n) => {
                                let regex = regex::Regex::new(r"\d{9}").unwrap();
                                Ok(regex.is_match(&n.to_string()))
                            }
                            None => Err(NoneError)
                        }
                    }?
                );
                is_valid
            }
        }
    }
    #[derive(Debug)]
    struct NoneError;
    
    impl std::error::Error for NoneError {}
    impl std::fmt::Display for NoneError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "No value found")
        }
    }
    
    pub fn count_valid_passports() -> usize{
        PASSPORT_DATA.split("\n\n").into_iter()
        .filter(|x| PassportData::new(x).has_required_fields())
        .count()
    }

    pub fn count_valid_passports_with_valid_data() -> usize {
        PASSPORT_DATA.split("\n\n").into_iter()
        .filter(|x| PassportData::new(x).has_fields_with_valid_data().unwrap_or(false))
        .count()

    }
}

pub mod day_five {
    const TICKETS: &str = super::constants::DAY_FIVE;
    fn get_seat_scores() -> Vec<usize> {
        TICKETS.lines().map(|x| {
            let row_binary_str = x[0..7].chars().fold(Vec::new(), |mut acc, current| {
                match current {
                    'F' => acc.push('0'),
                    'B' => acc.push('1'),
                    _ => {}
                }
                acc
            }).iter().collect::<String>();
            let row = usize::from_str_radix(&row_binary_str, 2).unwrap();
            let column_binary_str = x[7..10].chars().fold(Vec::new(), |mut acc, current| {
                match current {
                    'L' => acc.push('0'),
                    'R' => acc.push('1'),
                    _ => {}
                }
                acc
            }).iter().collect::<String>();
            let column = usize::from_str_radix(&column_binary_str, 2).unwrap();
            
            row * 8 + column
        }).collect()
    }
    pub fn get_max_seat_score() -> usize {
        *get_seat_scores().iter().max().unwrap()
    }

    pub fn get_my_seat_score() -> usize {
        let mut scores: Vec<usize> = get_seat_scores();
        scores.sort();

        let min = scores.iter().min().unwrap();
        scores.iter().fold(*min, |acc, current| {
            if acc + 1 == *current {
                *current
            } else { 
                acc
            }
        }) + 1
    }
}

pub mod day_six {
    use std::collections::HashMap;

    type IndividualsAnswers = Vec<(char, usize)>;
    const ANSWERS: &str = super::constants::DAY_SIX;

    pub fn get_answers_count_sum() -> usize {
        // step 1 - split string into groups
        ANSWERS.split("\n\n").map(|group_answers: &str| -> usize {
            // step 2 - split each group into a person
            let group = group_answers.lines().map(|persons_answer: &str| -> IndividualsAnswers {
                // step 3 - translate string into a Vec<(char, usize)>
                // 3.1 scan the string for characters
                let mut answer_tuple_vec: IndividualsAnswers = vec![];
                let mut found: bool = false;
                // 3.2 looping through each letter. should break when it finds a match or if theres nothing there
                for c in persons_answer.chars() {
                    // look through the vec for a redundant character
                    match answer_tuple_vec.iter().map(|x| x.0).position(|x| {x == c}) {
                        Some(i) => {
                            // if there is a duplicate, simply update the vec at that position and break
                            answer_tuple_vec[i] = (c, answer_tuple_vec[i].1 + 1);
                        },
                        None => {
                            // otherwise, add a new item to the vec
                            answer_tuple_vec.push((c, 1));
                        }
                    };
                }
                answer_tuple_vec
            }).fold(HashMap::new(), |mut acc: HashMap<char, usize>, current: IndividualsAnswers| {
                // translate it into a map so its easier to work with
                current.iter().inspect(|answer| {
                    match acc.get(&answer.0) {
                        Some(n) => {
                            acc.insert(answer.0, n + answer.1);
                        },
                        _ => {
                            acc.insert(answer.0, answer.1);
                        }
                    };
                }).count();
                acc
            }).keys().count();
            group
        }).fold(0, |acc: usize, current: usize| {
            acc + current
        })
    }

    pub fn get_all_answers() -> usize {
        ANSWERS.split("\n\n").map(|group_answers: &str| -> usize {
            // step 2 - split each group into a person
            let group = group_answers.lines().map(|persons_answer: &str| -> IndividualsAnswers {
                // step 3 - translate string into a Vec<(char, usize)>
                // 3.1 scan the string for characters
                let mut answer_tuple_vec: IndividualsAnswers = vec![];
                let mut found: bool = false;
                // 3.2 looping through each letter. should break when it finds a match or if theres nothing there
                for c in persons_answer.chars() {
                    // look through the vec for a redundant character
                    match answer_tuple_vec.iter().map(|x| x.0).position(|x| {x == c}) {
                        Some(i) => {
                            // if there is a duplicate, simply update the vec at that position and break
                            answer_tuple_vec[i] = (c, answer_tuple_vec[i].1 + 1);
                        },
                        None => {
                            // otherwise, add a new item to the vec
                            answer_tuple_vec.push((c, 1));
                        }
                    };
                }
                answer_tuple_vec
            }).fold(HashMap::new(), |mut acc: HashMap<char, usize>, current: IndividualsAnswers| {
                // translate it into a map so its easier to work with
                current.iter().inspect(|answer| {
                    match acc.get(&answer.0) {
                        Some(n) => {
                            acc.insert(answer.0, n + answer.1);

                        },
                        _ => {
                            acc.insert(answer.0, answer.1);
                        }
                    };
                }).count();
                // todo filter out entries where usize != current.length
                acc
            });
            
            group.iter()
            .filter(|x| {
                *x.1 == group_answers.lines().count()
            }).count()
        }).fold(0, |acc: usize, current: usize| {
            acc + current
        })
    }

}