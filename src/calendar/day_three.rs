use crate::constants;

pub const SLOPE_PATTERN: &str = constants::DAY_THREE;
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

