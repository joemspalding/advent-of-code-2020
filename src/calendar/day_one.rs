use crate::constants;


const EXPENSE_REPORT: [usize; 200] = constants::DAY_ONE;

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