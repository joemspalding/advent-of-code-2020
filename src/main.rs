use advent_of_code::*;
fn main() {
    println!("two numbers: {}", day_one::find_2020_product_two_num().unwrap());
    println!("three numbers: {}", day_one::find_2020_product_three_num().unwrap());
    println!("valid passwords: {}", day_two::find_valid_passwords());
    println!("REAL valid passwords: {}", day_two::find_valider_passwords());
    println!("trees in 1 down, 3 right: {}", day_three::find_trees());
    println!("trees in all patterns: {}", day_three::find_trees_all_patterns());
}
