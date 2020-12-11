use crate::constants;

const TICKETS: &str = constants::DAY_FIVE;
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
