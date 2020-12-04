use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

macro_rules! parse_if_some {
    ($src:expr, $val:ident, $parse:expr) => {
        match $src {
            None => None,
            Some($val) => ($parse),
        }
    };
}

macro_rules! res_to_opt {
    ($parse:expr) => {
        ($parse).map_or(None, Some)
    };
}

macro_rules! parse_num {
    ($src:expr, $digits:expr) => {
        parse_if_some!(
            $src,
            val,
            match val.len() {
                $digits => res_to_opt!(val.parse()),
                _ => None,
            }
        )
    };
}

struct PassportData {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl PassportData {
    fn new() -> Self {
        PassportData {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None,
        }
    }

    fn has_required_fields(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn to_passport(&self) -> Passport {
        Passport {
            birth_year: parse_num!(&self.birth_year, 4),
            issue_year: parse_num!(&self.issue_year, 4),
            expiration_year: parse_num!(&self.expiration_year, 4),
            height: parse_if_some!(
                &self.height,
                val,
                res_to_opt!(Height::from_str(val.as_str()))
            ),
            hair_color: parse_if_some!(
                &self.hair_color,
                val,
                match (val.starts_with("#"), val.len()) {
                    (true, 7) => res_to_opt!(u32::from_str_radix(&val[1..], 16)),
                    _ => None,
                }
            ),
            eye_color: parse_if_some!(
                &self.eye_color,
                val,
                res_to_opt!(EyeColor::from_str(val.as_str()))
            ),
            passport_id: parse_num!(&self.passport_id, 9),
            country_id: parse_if_some!(&self.country_id, val, Some(val.clone())),
        }
    }
}

enum Height {
    Cm(usize),
    In(usize),
}

impl FromStr for Height {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 3 {
            return Err(());
        }
        let (value, suffix) = s.split_at(s.len() - 2);
        match suffix {
            "cm" => Ok(Height::Cm(value.parse().unwrap())),
            "in" => Ok(Height::In(value.parse().unwrap())),
            _ => Err(()),
        }
    }
}

enum EyeColor {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}

impl FromStr for EyeColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" => Ok(EyeColor::Amber),
            "blu" => Ok(EyeColor::Blue),
            "brn" => Ok(EyeColor::Brown),
            "gry" => Ok(EyeColor::Grey),
            "grn" => Ok(EyeColor::Green),
            "hzl" => Ok(EyeColor::Hazel),
            "oth" => Ok(EyeColor::Other),
            _ => Err(()),
        }
    }
}

struct Passport {
    birth_year: Option<usize>,
    issue_year: Option<usize>,
    expiration_year: Option<usize>,
    height: Option<Height>,
    hair_color: Option<u32>,
    eye_color: Option<EyeColor>,
    passport_id: Option<usize>,
    country_id: Option<String>,
}

impl Passport {
    fn has_valid_birth_year(&self) -> bool {
        match self.birth_year {
            None => false,
            Some(value) => (1920..=2002).contains(&value),
        }
    }

    fn has_valid_issue_year(&self) -> bool {
        match self.issue_year {
            None => false,
            Some(value) => (2010..=2020).contains(&value),
        }
    }

    fn has_valid_expiration_year(&self) -> bool {
        match self.expiration_year {
            None => false,
            Some(value) => (2020..=2030).contains(&value),
        }
    }

    fn has_valid_height(&self) -> bool {
        match &self.height {
            None => false,
            Some(height) => match height {
                Height::Cm(height) => (150..=193).contains(height),
                Height::In(height) => (59..=76).contains(height),
            },
        }
    }

    fn has_valid_hair_color(&self) -> bool {
        match &self.hair_color {
            None => false,
            Some(hair_color) => *hair_color <= 0xffffff, // 24bit color
        }
    }

    fn has_valid_eye_color(&self) -> bool {
        self.eye_color.is_some()
    }

    fn has_valid_passport_id(&self) -> bool {
        match &self.passport_id {
            None => false,
            Some(passport_id) => *passport_id < 1_000_000_000, // 9-digit number
        }
    }

    fn has_valid_country_id(&self) -> bool {
        self.country_id.is_none() || self.country_id.is_some()
    }

    fn is_valid(&self) -> bool {
        self.has_valid_birth_year()
            && self.has_valid_issue_year()
            && self.has_valid_expiration_year()
            && self.has_valid_height()
            && self.has_valid_hair_color()
            && self.has_valid_eye_color()
            && self.has_valid_passport_id()
            && self.has_valid_country_id()
    }
}

#[aoc_generator(day4)]
fn parse_input_day4(input: &str) -> Vec<PassportData> {
    input
        .split("\n\n")
        .map(|item| {
            item.split(&['\n', ' '][..])
                .map(|pair| pair.split(':').collect::<Vec<&str>>())
                .fold(PassportData::new(), |mut data, pair| {
                    let (key, value) = (pair[0], pair[1].to_string());
                    match key {
                        "byr" => data.birth_year = Some(value),
                        "iyr" => data.issue_year = Some(value),
                        "eyr" => data.expiration_year = Some(value),
                        "hgt" => data.height = Some(value),
                        "hcl" => data.hair_color = Some(value),
                        "ecl" => data.eye_color = Some(value),
                        "pid" => data.passport_id = Some(value),
                        "cid" => data.country_id = Some(value),
                        _ => {}
                    };
                    data
                })
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[PassportData]) -> usize {
    input
        .iter()
        .filter(|data| data.has_required_fields())
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &[PassportData]) -> usize {
    input
        .iter()
        .filter(|data| data.to_passport().is_valid())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input_day4(INPUT)), 2);
    }

    #[test]
    fn part2_valids() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(part2(&parse_input_day4(input)), 4);
    }
    #[test]
    fn part2_invalids() {
        let input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        assert_eq!(part2(&parse_input_day4(input)), 0);
    }
}
