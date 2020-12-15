use crate::constants;
use regex::Regex;
const BAG_RULES: &str = constants::DAY_SEVEN;

#[derive(Debug, Clone)]
struct Bag {
    color: String,
    contents: Option<Vec<(String, usize)>>
}

pub fn find_bags_that_have_shiny_gold_bag() -> usize{
    let main_color_regex = Regex::new(r"(.+?) bags contain (.+?)\.").unwrap();
    let inside_parser_regex = Regex::new(r"(\d+) (.+?) (bags)").unwrap();
    BAG_RULES.lines()
    .map(|bag_rule| {
        let match_rule = main_color_regex.captures(bag_rule).unwrap();
        // println!("{:?}", color.get(0));
        println!("color {:?}", match_rule.get(1).unwrap().as_str());
        println!("insides {:?}", match_rule.get(2).unwrap().as_str());
        let color = match_rule.get(1).unwrap().as_str();
        let contents: Option<Vec<(String, usize)>> = match match_rule.get(2) {
            Some(n) => {
                Some(n.as_str().split(", ").map(|rule| {
                    let inside_match = inside_parser_regex.captures(rule).unwrap();
                    (
                        inside_match.get(1).unwrap().as_str().to_string(),
                        inside_match.get(2).unwrap().as_str().parse().unwrap()
                    )
                }).collect())
            },
            None => None
        };

        Bag {
            color: color.to_string(),
            contents
        }
    }).inspect(|x| {
        println!("{:?}", x);
    })
    .count()
}