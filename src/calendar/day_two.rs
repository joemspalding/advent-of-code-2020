use crate::constants;

const CORRUPTED_PASSWORDS: [(usize, usize, char, &str); 1000] = constants::DAY_TWO;

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