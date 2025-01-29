use std::fmt;
use crate::charToDigit;
use crate::isDigit;

// UInt
#[derive(Clone)]
pub struct UInt(String);
impl UInt {
    pub fn new(value: &str) -> Self {
        if isDigit(value) {
            UInt(value.to_string())
        } else {
            UInt("0".to_string())
        }
    }
}
impl fmt::Display for UInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
// +
impl std::ops::Add for UInt {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut leftChars  = (self.0).chars().rev().peekable();
        let mut rightChars = (other.0).chars().rev().peekable();

        let mut carry = 0;
        let mut result: Vec<u8> = Vec::new();

        while leftChars.peek().is_some() || rightChars.peek().is_some() || carry != 0 {
            let leftDigit  = leftChars.next().map_or(0, charToDigit);
            let rightDigit = rightChars.next().map_or(0, charToDigit);

            let sum = leftDigit + rightDigit + carry;
            result.push(sum % 10);
            carry = sum / 10;
        }

        UInt( result.iter().rev().map(|&digit| (digit + b'0') as char).collect() )
    }
}
// -
impl std::ops::Sub for UInt {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut leftChars  = (self.0).chars().rev().peekable();
        let mut rightChars = (other.0).chars().rev().peekable();

        // сheck if result will be negative
        if self.0 < other.0 {
            return UInt("0".to_string());
        }

        let mut borrow = 0;
        let mut result: Vec<u8> = Vec::new();

        while leftChars.peek().is_some() || rightChars.peek().is_some() {
            let leftDigit  = leftChars.next().map_or(0, charToDigit);
            let rightDigit = rightChars.next().map_or(0, charToDigit);

            let mut diff = leftDigit as i8 - rightDigit as i8 - borrow;

            if diff < 0 {
                diff += 10;
                borrow = 1;
            } else {
                borrow = 0;
            }

            result.push(diff as u8);
        }

        while let Some(&0) = result.last() {
            result.pop();
        }

        if result.is_empty() {
            result.push(0);
        }

        UInt( result.iter().rev().map(|&digit| (digit + b'0') as char).collect() )
    }
}
// *
impl std::ops::Mul for UInt {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut result = vec![0; self.0.len() + other.0.len()];

        for (i, leftChar) in self.0.chars().rev().enumerate() {
            let leftDigit = charToDigit(leftChar);
            let mut carry = 0;

            for (j, rightChar) in other.0.chars().rev().enumerate() {
                let rightDigit = charToDigit(rightChar);
                let sum = leftDigit * rightDigit + result[i + j] + carry;
                result[i + j] = sum % 10;
                carry = sum / 10;
            }

            if carry > 0 {
                result[i + other.0.len()] += carry;
            }
        }

        while let Some(&0) = result.last() {
            result.pop();
        }

        if result.is_empty() {
            result.push(0);
        }

        UInt(result.iter().rev().map(|&digit| (digit + b'0') as char).collect())
    }
}
// /
impl std::ops::Div for UInt {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        println!("!: {} {}",self.0,other.0);
        if other.0 == "0" {
            return UInt(self.0);
        }

        let mut result = String::new();
        let mut remainder = UInt::new("0");

        println!("c {:?}",self.0.chars());
        for digit in self.0.chars() {
            println!("{}*{}+{}",remainder,10,digit);
            remainder = remainder * UInt::new("10") + UInt::new(&(digit.to_string()));
            println!("  = {}",remainder);

            let mut quotient_digit = 0;
            println!("  ? {} {} = {}",remainder.0,other.0,remainder.0 >= other.0);
            while remainder.0 >= other.0 {
                remainder = remainder.clone() - other.clone();
                quotient_digit += 1;
            }
            result.push((quotient_digit + b'0') as char);
        }

        while result.starts_with('0') && result.len() > 1 {
            result.remove(0);
        }

        UInt(result)
    }
}