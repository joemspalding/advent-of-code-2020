use crate::constants;
use std::collections::HashMap;

type IndividualsAnswers = Vec<(char, usize)>;

const ANSWERS: &str = constants::DAY_SIX;

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
