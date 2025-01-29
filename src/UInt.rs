use std::fmt;

fn isDigit(c: u8) -> bool {        // todo: use from tokenizer.rs
    c >= b'0' && c <= b'9'
}
fn isUInt(value: &str) -> bool {   // todo: add this to tokenizer.rs
    value.bytes().all(|c| isDigit(c))
}

use crate::DInt::*;
use crate::DUInt::*;

// UInt
#[derive(Clone)]
pub struct UInt(Vec<DUInt>); // value
impl UInt {
    pub fn new(value: &str) -> Self {
        if isUInt(value) {
            Self( value.chars().rev().map(charToDUInt).collect() )
        } else {
            Self( vec![DUInt(0)] )
        }
    }
}
impl fmt::Display for UInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.0.iter().rev().map(|d| (d.0 + b'0') as char).collect();
        write!(f, "{}", s)
    }
}
// +
impl std::ops::Add for UInt {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut leftDigits  = self.0.iter().cloned().peekable();
        let mut rightDigits = other.0.iter().cloned().peekable();

        // num.len < num.len
        if leftDigits.len() < rightDigits.len() {
            std::mem::swap(&mut leftDigits, &mut rightDigits);
        }

        //
        let mut result: Vec<DUInt> = Vec::new();

        let mut leftDigit:  DUInt;
        let mut rightDigit: DUInt;

        let mut carry: u8 = 0;
        let mut sum:   u8;

        while leftDigits.peek().is_some() || rightDigits.peek().is_some() || carry != 0 {
            leftDigit  = leftDigits.next().unwrap_or(DUInt(0));
            rightDigit = rightDigits.next().unwrap_or(DUInt(0));

            sum = leftDigit.0+rightDigit.0 + carry;
            result.push( DUInt::u8(sum%10) );
            carry = sum/10;
        }

        // next result
        Self(result)
    }
}
// -
impl std::ops::Sub for UInt {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut leftDigits  = self.0.iter().cloned().peekable();
        let mut rightDigits = other.0.iter().cloned().peekable();

        // check if result will be negative
        if DUIntGreater(&self.0, &other.0, true) != std::cmp::Ordering::Greater {
            return Self( vec![DUInt(0)] );
        }

        //
        let mut borrow: DUInt      = DUInt(0);
        let mut result: Vec<DUInt> = Vec::new();
        let mut diff:   DInt;

        while leftDigits.peek().is_some() || rightDigits.peek().is_some() {
            let leftDigit:  DUInt = leftDigits.next().unwrap_or(DUInt(0));
            let rightDigit: DUInt = rightDigits.next().unwrap_or(DUInt(0));

            diff = DInt(leftDigit.0 as i8 -rightDigit.0 as i8 -borrow.0 as i8);
            if diff.0 < 0 {
                diff.0 += 10;
                borrow.0 = 1;
            } else {
                borrow.0 = 0;
            }
            result.push( DUInt(diff.0 as u8) );
        }

        // next result
        unnecessaryIntResult(&mut result);
        Self(result)
    }
}
// *
impl std::ops::Mul for UInt {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        // num * 0 || 0 * num
        if checkZeroValue(&other.0) || checkZeroValue(&self.0) {
            return Self( vec![DUInt(0)] );
        }

        //
        let mut result: Vec<DUInt> = vec![DUInt(0); self.0.len()+other.0.len()];
        let mut carry: DUInt;
        for (i, &leftDigit) in self.0.iter().enumerate() {
            carry = DUInt(0);
            for (j, &rightDigit) in other.0.iter().enumerate() {
                let sum: u8 = leftDigit.0 *rightDigit.0 +result[i+j].0 +carry.0; // todo: move to up
                result[i+j].0 = sum % 10;
                carry.0 = sum / 10;
            }

            if carry.0 > 0 {
                result[i+other.0.len()] += carry;
            }
        }

        // next result
        unnecessaryIntResult(&mut result);
        Self(result)
    }
}
// /
impl std::ops::Div for UInt {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        println!("\nUInt /");
        // if right op = 0
        if checkZeroValue(&other.0) {
            return Self(self.0);
        }

        //
        let mut result: Vec<DUInt> = Vec::new();
        let mut remainder = Self( vec![DUInt(0)] );

        for &digit in self.0.iter().rev() {
            let mut quotientDigit: usize = 0; // todo: move to up
            remainder = remainder *Self(vec![DUInt(10)]) +Self(vec![digit]);

            while {
                let order = DUIntGreater(&remainder.0, &other.0, true); // todo: move to up
                order == std::cmp::Ordering::Greater || order == std::cmp::Ordering::Equal
            } {
                remainder = remainder.clone() - other.clone();
                quotientDigit += 1;
            }
            result.push( DUInt(quotientDigit as u8) );
        }

        // next result
        result.reverse();
        unnecessaryIntResult(&mut result);
        Self(result)
    }
}
// check value = 0
fn checkZeroValue(result: &Vec<DUInt>) -> bool { // todo: merge this
    *result == vec![DUInt(0)]
}
// remove unnecessary result
fn unnecessaryIntResult(result: &mut Vec<DUInt>) { // todo: merge this INT ONLY
    // remove some 0
    while let Some(&DUInt(0)) = result.last() {
        result.pop();
    }
    // empty = 0
    if result.is_empty() {
        *result = vec![DUInt(0)];
    }
}