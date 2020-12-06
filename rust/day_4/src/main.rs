use common::get_file_content;
use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = get_file_content(args.last().expect("Failed to get args").to_string())
        .expect("Failed to get file content");
    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
/*
byr (Birth Year)
iyr (Issue Year)
eyr (Expiration Year)
hgt (Height)
hcl (Hair Color)
ecl (Eye Color)
pid (Passport ID)
cid (Country ID)
*/
#[derive(Default)]
struct PassportInfo {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl PassportInfo {
    pub fn new(codes: &String) -> PassportInfo {
        let mut pass_info = PassportInfo {
            ..Default::default()
        };
        for code in codes.split(' ').collect::<Vec<&str>>() {
            let a = code.split(':').collect::<Vec<&str>>();
            match a[0] {
                "byr" => pass_info.byr = Some(a[1].to_string()),
                "iyr" => pass_info.iyr = Some(a[1].to_string()),
                "eyr" => pass_info.eyr = Some(a[1].to_string()),
                "hgt" => pass_info.hgt = Some(a[1].to_string()),
                "hcl" => pass_info.hcl = Some(a[1].to_string()),
                "ecl" => pass_info.ecl = Some(a[1].to_string()),
                "pid" => pass_info.pid = Some(a[1].to_string()),
                "cid" => pass_info.cid = Some(a[1].to_string()),
                _ => {}
            }
        }
        pass_info
    }

    pub fn is_valid_1(&self) -> bool {
        if self.byr == None {
            return false;
        }
        if self.iyr == None {
            return false;
        }
        if self.eyr == None {
            return false;
        }
        if self.hgt == None {
            return false;
        }
        if self.hcl == None {
            return false;
        }
        if self.ecl == None {
            return false;
        }
        if self.pid == None {
            return false;
        }
        true
    }

    pub fn is_valid_2(&self) -> bool {
        if self.byr == None {
            return false;
        } else if let Some(v) = &self.byr {
            match v.parse::<i32>() {
                Ok(n) => {
                    if n < 1920 || n > 2002 {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        if self.iyr == None {
            return false;
        } else if let Some(v) = &self.iyr {
            match v.parse::<i32>() {
                Ok(n) => {
                    if n < 2010 || n > 2020 {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        if self.eyr == None {
            return false;
        } else if let Some(v) = &self.eyr {
            match v.parse::<i32>() {
                Ok(n) => {
                    if n < 2020 || n > 2030 {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        if self.hgt == None {
            return false;
        } else if let Some(v) = &self.hgt {
            let n = v[0..v.len() - 2].parse::<i32>().unwrap_or_else(|_| -1);
            match &v[v.len() - 2..v.len()] {
                "in" => {
                    if n < 59 || n > 76 {
                        return false;
                    }
                }
                "cm" => {
                    if n < 150 || n > 193 {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        if self.hcl == None {
            return false;
        } else if let Some(v) = &self.hcl {
            let re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
            if !re.is_match(&v) {
                return false;
            }
        }
        if self.ecl == None {
            return false;
        } else if let Some(v) = &self.ecl {
            match &v[..] {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
                _ => return false,
            }
        }
        if self.pid == None {
            return false;
        } else if let Some(v) = &self.pid {
            let re = Regex::new(r"^[0-9]{9}$").unwrap();
            if !re.is_match(&v) {
                return false;
            }
        }
        true
    }
}

fn part_1(contents: &String) -> i32 {
    let lines = contents.split("\n").collect::<Vec<&str>>();

    let mut values = String::from("");
    let mut count = 0;
    for line in lines {
        if line == "" {
            if PassportInfo::new(&values).is_valid_1() {
                count += 1;
            }
            values = "".to_string();
        }
        values = format!("{} {}", values, line);
    }
    if PassportInfo::new(&values).is_valid_1() {
        count += 1;
    }
    count
}

fn part_2(contents: &String) -> i32 {
    let lines = contents.split("\n").collect::<Vec<&str>>();

    let mut values = String::from("");
    let mut count = 0;
    for line in lines {
        if line == "" {
            if PassportInfo::new(&values).is_valid_2() {
                count += 1;
            }
            values = "".to_string();
        }
        values = format!("{} {}", values, line);
    }
    if PassportInfo::new(&values).is_valid_2() {
        count += 1;
    }
    count
}
