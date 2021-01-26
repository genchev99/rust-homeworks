use std::cmp::*;
use std::str::FromStr;
use std::ops::{Add, Sub};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub struct Bigint {
    sign: i8,
+
    digits: Vec<u8>,
}

impl Bigint {
    pub fn new() -> Self {
        return Bigint { sign: i8::MAX, digits: Vec::new() };
    }

    fn get_inversed(&self) -> Self {
        Self {
            sign: if self.sign == i8::MAX {i8::MIN} else {i8::MAX},
            digits: self.digits.clone(),
        }
    }

    fn get_abs(&self) -> Self {
        Self {
            sign: i8::MAX,
            digits: self.digits.clone(),
        }
    }

    fn get_size(&self) -> usize {
        self.digits.len()
    }

    fn get_sign_as_char(&self) -> char {
        return if self.sign == i8::MAX { '+' } else { '-' }
    }

    pub fn print(&self) {
        println!("{:?} {:?}", self.get_sign_as_char(), self.digits)
    }

    pub fn is_positive(&self) -> bool {
        return self.sign == i8::MAX;
    }

    pub fn is_negative(&self) -> bool {
        return self.sign == i8::MIN;
    }
}

#[derive(Debug)]
pub struct ParseError;

impl FromStr for Bigint {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits: Vec<u8> = Vec::new();
        let mut sign: i8 = i8::MAX;
        let mut init_index = 0;
        let chars: Vec<char> = s.chars().collect();

        if chars.len() != 0 {
            if chars[0] == '+' {
                sign = i8::MAX;
                init_index = 1;
            } else if chars[0] == '-' {
                sign = i8::MIN;
                init_index = 1;
            }

            for i in init_index..chars.len() {
                let char = chars[i];

                if !char.is_digit(10) {
                    return Err(ParseError);
                }

                digits.push(char.to_digit(10).unwrap() as u8)
            }

            while !digits.is_empty() && digits[0] == 0 {
                digits.remove(0);
            }
        }

        if digits.len() == 0 {
            sign = i8::MAX;
        }

        Ok(Bigint { digits, sign })
    }
}

impl PartialOrd for Bigint {
    fn partial_cmp(&self, other: &Bigint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bigint {
    fn cmp(&self, other: &Bigint) -> Ordering {
        if self.sign != other.sign {
            if self.sign < other.sign {
                return Ordering::Less;
            } else if self.sign > other.sign {
                return Ordering::Greater;
            }
        }

        let is_negative: bool = self.is_negative();

        if self.get_size() != other.get_size() {
            if self.get_size() < other.get_size() {
                return if is_negative { Ordering::Greater } else { Ordering::Less };
            } else if self.get_size() > other.get_size() {
                return if is_negative { Ordering::Less } else { Ordering::Greater };
            }
        }

        return self.digits.cmp(&other.digits);
    }
}

fn add_digits(mut left: Vec<u8>, mut right: Vec<u8>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    let bigger_length = std::cmp::max(left.len(), right.len());
    let mut left_reversed: Vec<u8> = left.clone();
    let mut right_reversed: Vec<u8> = right.clone();

    left_reversed.reverse();
    right_reversed.reverse();
    // pad with zeros
    left_reversed.append(&mut vec![0; bigger_length - left.len()]);
    right_reversed.append(&mut vec![0; bigger_length - right.len()]);

    let mut carrier:u8 = 0;
    for i in 0..bigger_length {
        res.push((left_reversed[i] + right_reversed[i] + carrier.clone())  % 10);
        carrier = (left_reversed[i] + right_reversed[i] + carrier) / 10;
    }

    if carrier != 0 {
        res.push(carrier);
    }

    res.reverse();

    return res;
}

fn subtract_digits(mut larger: Vec<u8>, mut smaller: Vec<u8>) -> Vec<u8>  {
    let mut res: Vec<u8> = Vec::new();
    let bigger_length = std::cmp::max(larger.len(), smaller.len());
    let mut larger_reversed: Vec<u8> = larger.clone();
    let mut smaller_reversed: Vec<u8> = smaller.clone();

    larger_reversed.reverse();
    smaller_reversed.reverse();
    // pad with zeros
    larger_reversed.append(&mut vec![0; bigger_length - larger.len()]);
    smaller_reversed.append(&mut vec![0; bigger_length - smaller.len()]);

    let mut carrier:u8 = 0;
    for i in 0..bigger_length {
        let sub: u8 = 10 + larger_reversed[i] - smaller_reversed[i];

        res.push((sub.clone() - carrier.clone()) % 10);
        carrier = ((sub - carrier) < 10) as u8
    }

    res.reverse();

    // remove padding
    while !res.is_empty() && res[0] == 0 {
        res.remove(0);
    }

    return res;
}

impl Add for Bigint {
    type Output = Bigint;

    fn add(self, other: Self) -> Self {
        if self.sign == other.sign {
            return Self {
                sign: self.sign.clone(),
                digits: add_digits(self.digits, other.digits),
            }
        }

        let bigger: Vec<u8> = if self.get_abs() > other.get_abs() { self.digits.clone() } else { other.digits.clone() };
        let smaller: Vec<u8> = if self.get_abs() < other.get_abs() { self.digits.clone() } else { other.digits.clone() };

        let res: Vec<u8> = subtract_digits(bigger, smaller);

        return Self {
            sign: if res.is_empty() { i8::MAX } else if self.get_abs() > other.get_abs() { self.sign.clone() } else { other.sign.clone() },
            digits: res,
        }
    }
}

impl Sub for Bigint {
    type Output = Bigint;

    fn sub(self, other: Self) -> Self {
        if self.sign != other.sign {
            return Self {
                sign: self.sign.clone(),
                digits: add_digits(self.get_inversed().digits, other.digits)
            }
        }

        return self + other.get_inversed()
    }
}

