use crate::constants;

const PASSPORT_DATA: &str = constants::DAY_FOUR;
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
