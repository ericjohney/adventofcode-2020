use adventofcode2019::file_utils;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let passports = file_utils::read_to_string("inputs/day4.txt")
        .trim()
        .split("\n\n")
        .map(|passport| {
            let mut map = HashMap::new();
            passport.split_whitespace().for_each(|m| {
                let pairs: Vec<String> = m.split(":").map(|s| s.into()).collect();
                println!("{} {}", pairs[0], pairs[1]);
                map.insert(pairs[0].to_string(), pairs[1].to_string());
            });
            println!("----------");
            return map;
        })
        .collect::<Vec<HashMap<String, String>>>();

    part1(&passports);
    part2(&passports);
}

fn part1(passports: &Vec<HashMap<String, String>>) {
    let mut valid_count = 0;
    for passport in passports {
        if passport.contains_key("cid") && passport.keys().len() == 8 {
            valid_count += 1;
        } else if !passport.contains_key("cid") && passport.keys().len() == 7 {
            valid_count += 1;
        }
    }
    println!("part1 valid count {}", valid_count);
}

fn part2(passports: &Vec<HashMap<String, String>>) {
    fn passport_is_valid(passport: &HashMap<String, String>) -> bool {
        if passport.contains_key("cid") && passport.keys().len() < 8 {
            return false;
        }
        if !passport.contains_key("cid") && passport.keys().len() < 7 {
            return false;
        }

        let byr = str::parse::<i64>(passport.get("byr").unwrap()).unwrap();
        if !(byr >= 1920 && byr <= 2002) {
            return false;
        }
        println!("byr {}", byr);

        let iyr = str::parse::<i64>(passport.get("iyr").unwrap()).unwrap();
        if !(iyr >= 2010 && iyr <= 2020) {
            return false;
        }
        println!("iyr {}", iyr);

        let eyr = str::parse::<i64>(passport.get("eyr").unwrap()).unwrap();
        if !(eyr >= 2020 && eyr <= 2030) {
            return false;
        }
        println!("eyr {}", eyr);

        if !(passport.get("hgt").unwrap().ends_with("in")
            || passport.get("hgt").unwrap().ends_with("cm"))
        {
            return false;
        }
        if passport.get("hgt").unwrap().ends_with("in") {
            let hgt = str::parse::<i64>(passport.get("hgt").unwrap().strip_suffix("in").unwrap())
                .unwrap();
            if !(hgt >= 59 && hgt <= 76) {
                return false;
            }
        }
        if passport.get("hgt").unwrap().ends_with("cm") {
            let hgt = str::parse::<i64>(passport.get("hgt").unwrap().strip_suffix("cm").unwrap())
                .unwrap();
            if !(hgt >= 150 && hgt <= 193) {
                return false;
            }
        }

        let hcl = passport.get("hcl").unwrap();
        let re_hcl = Regex::new(r"^#[0-9A-Fa-f]{6}$").unwrap();
        if !re_hcl.is_match(hcl) {
            return false;
        }

        let ecl = passport.get("ecl").unwrap();
        let re_ecl = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        if !re_ecl.is_match(ecl) {
            return false;
        }

        let pid = passport.get("pid").unwrap();
        let re_pid = Regex::new(r"^\d{9}$").unwrap();
        if !re_pid.is_match(pid) {
            return false;
        }

        return true;
    }

    let mut valid_count = 0;
    for passport in passports {
        if passport_is_valid(passport) {
            valid_count += 1;
        }
    }

    println!("part2 valid count {}", valid_count);
}
