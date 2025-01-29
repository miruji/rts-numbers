use std::fmt;
use crate::isDigit;
pub fn charToDigit(c: char) -> u8 {
    match c {
        '0'..='9' => c as u8 - b'0',
        _ => 0,
    }
}
// UInt
#[derive(Clone)]
pub struct UInt(Vec<u8>);
impl UInt {
    pub fn new(value: &str) -> Self {
        if isDigit(value) {
            let digits = value.chars().rev().map(charToDigit).collect();
            Self(digits)
        } else {
            Self(vec![0])
        }
    }
}
impl fmt::Display for UInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.0.iter().rev().map(|&d| (d + b'0') as char).collect();
        write!(f, "{}", s)
    }
}
// +
impl std::ops::Add for UInt {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut leftDigits  = self.0.iter().peekable();
        let mut rightDigits = other.0.iter().peekable();

        let mut carry = 0;
        let mut result: Vec<u8> = Vec::new();

        while leftDigits.peek().is_some() || rightDigits.peek().is_some() || carry != 0 {
            let leftDigit  = leftDigits.next().map_or(0, |&d| d);
            let rightDigit = rightDigits.next().map_or(0, |&d| d);

            let sum = leftDigit + rightDigit + carry;
            result.push(sum % 10);
            carry = sum / 10;
        }

        UInt(result)
    }
}
// -
impl std::ops::Sub for UInt {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut leftDigits  = self.0.iter().peekable();
        let mut rightDigits = other.0.iter().peekable();

        // Check if result will be negative
        if self.0.len() < other.0.len() || (self.0.len() == other.0.len() && 
           self.0.iter().rev().cloned().collect::<Vec<u8>>() < other.0.iter().rev().cloned().collect::<Vec<u8>>()) {
            return UInt(vec![0]);
        }

        let mut borrow = 0;
        let mut result: Vec<u8> = Vec::new();
        let mut diff;

        while leftDigits.peek().is_some() || rightDigits.peek().is_some() {
            let leftDigit  = leftDigits.next().map_or(0, |&d| d);
            let rightDigit = rightDigits.next().map_or(0, |&d| d);

            diff = leftDigit as i8 - rightDigit as i8 - borrow;

            if diff < 0 {
                diff += 10;
                borrow = 1;
            } else {
                borrow = 0;
            }

            result.push(diff as u8);
        }

        unnecessaryResult(&mut result);
        UInt(result)
    }
}
// *
impl std::ops::Mul for UInt {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut result = vec![0; self.0.len() + other.0.len()];

        for (i, &leftDigit) in self.0.iter().enumerate() {
            let mut carry = 0;

            for (j, &rightDigit) in other.0.iter().enumerate() {
                let sum = leftDigit * rightDigit + result[i + j] + carry;
                result[i + j] = sum % 10;
                carry = sum / 10;
            }

            if carry > 0 {
                result[i + other.0.len()] += carry;
            }
        }

        unnecessaryResult(&mut result);
        UInt(result)
    }
}
// /
impl std::ops::Div for UInt {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if other.0 == vec![0] {
            return UInt(self.0);
        }
        let right = other.0.iter().rev().cloned().collect::<Vec<u8>>();

        let mut result = Vec::new();
        let mut remainder = UInt(vec![0]);

        for &digit in self.0.iter().rev() {
            println!("{}*{}+{}",remainder,UInt(vec![10]),UInt(vec![digit]));
            remainder = remainder * UInt(vec![10]) + UInt(vec![digit]);

            let mut quotient_digit = 0;
            println!("  ={}",remainder);
            println!("  {}",remainder.0.len() > other.0.len() || (remainder.0.len() == other.0.len() &&
                  remainder.0.iter().rev().cloned().collect::<Vec<u8>>() >= other.0.iter().rev().cloned().collect::<Vec<u8>>()));
            while remainder.0.len() > other.0.len() || (remainder.0.len() == other.0.len() &&
                  remainder.0.iter().rev().cloned().collect::<Vec<u8>>() >= other.0.iter().rev().cloned().collect::<Vec<u8>>()) {
                println!("    {} - {}",remainder.clone(),other.clone());
                remainder = remainder.clone() - other.clone();
                println!("    ={}",remainder);
                println!("    quotient_digit {}",quotient_digit);
                quotient_digit += 1;
            }
            println!("  === {}",quotient_digit);
            result.push(quotient_digit);
        }

        result.reverse();
        unnecessaryResult(&mut result);
        UInt(result)
    }
}
fn unnecessaryResult(mut result: &mut Vec<u8>) {
    while let Some(&0) = result.last() {
        result.pop();
    }

    if result.is_empty() {
        result.push(0);
    }
}
