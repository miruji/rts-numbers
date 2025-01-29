use std::fmt;

fn isDigit(c: u8) -> bool {     // todo: use from tokenizer.rs
    c >= b'0' && c <= b'9'
}
fn isInt(value: &str) -> bool { // todo: add this to tokenizer.rs
    let mut bytes = value.bytes();
    if let Some(firstChar) = bytes.next() {
        if firstChar != b'-' && !isDigit(firstChar) {
            return false;
        }
    }
    bytes.all(|c| isDigit(c))
}
use crate::DInt::*;
use crate::DUInt::*;

// Int
#[derive(Clone)]
pub struct Int(Vec<DUInt>,bool); // value - negative
impl Int {
    pub fn new(value: &str) -> Self {
        if isInt(value) {
            let mut digits: Vec<_> = value.chars().rev().map(charToDUInt).collect();
            let mut negative: bool = false;
            if digits.last().unwrap().0 == 0 {
                if digits.len() > 1 && digits.iter().rev().nth(1).unwrap().0 != 0 {
                    negative = true;
                }
                if digits.len() != 1 {
                    digits.remove(digits.len()-1);
                }
            }
            Self( digits, negative )
        } else {
            Self( vec![DUInt(0)], false )
        }
    }
}
impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.0.iter().rev().map(|d| (d.0 + b'0') as char).collect();
        if self.1 {
            write!(f, "-{}", s)
        } else {
            write!(f, "{}", s)
        }
    }
}
// +
impl std::ops::Add for Int {
    type Output = Self;
    fn add(self, mut other: Self) -> Self {
        // if right op = 0
        if checkZeroValue(&other.0) {
            return self;
        }

        // negaive
        let mut negative: bool = false;
        // ?num + -num
        if other.1 {
            // -num + -num
            if self.1 {
                negative = true;
            // +num + -num
            } else {
                other.1 = false;
                return self-other;
            }
        } else
        // -num + +num
        if self.1 {
            other.1 = true;
            return self-other;
        }

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
        // check negative zero result
        if checkZeroValue(&result) && negative {
            negative = false;
        }
        Self(result, negative)
    }
}
// -
impl std::ops::Sub for Int {
    type Output = Self;
    fn sub(self, mut other: Self) -> Self {
        // if right op = 0
        if checkZeroValue(&other.0) {
            return self;
        }

        // negaive
        let mut negative: bool = false;
        // ?num - -num
        if other.1 {
            // -num - -num
            if self.1 {
                other.1 = false;
                negative = true;
            // +num - -num
            } else {
                other.1 = false;
                return self+other;
            }
        } else
        // -num - +num
        if self.1 {
            other.1 = true;
            return self+other;
        }

        let mut leftDigits  = self.0.iter().cloned().peekable();
        let mut rightDigits = other.0.iter().cloned().peekable();

        // check if result will be negative
        if DUIntGreater(&self.0, &other.0, true) != std::cmp::Ordering::Greater {
            std::mem::swap(&mut leftDigits, &mut rightDigits);
            negative = true;
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
        // check negative zero result
        if checkZeroValue(&result) && negative {
            negative = false;
        }
        Self(result, negative)
    }
}
// *
impl std::ops::Mul for Int {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        // num * 0 || 0 * num
        if checkZeroValue(&other.0) || checkZeroValue(&self.0) {
            return Self( vec![DUInt(0)], false );
        }

        // negaive
        let mut negative: bool = false;
        // ?num * -num
        if other.1 == true {
            // num * -num
            if self.1 != true {
                negative = true;
            }
        } else
        // -num * +num
        if self.1 {
            negative = true;
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
        // no checkZeroValue, impossible
        Self(result, negative)
    }
}
// /
impl std::ops::Div for Int {
    type Output = Self;
    fn div(self, mut other: Self) -> Self {
        // if right op = 0
        if checkZeroValue(&other.0) {
            return Self(self.0, self.1);
        } else
        if checkZeroValue(&self.0) {
            return Self( vec![DUInt(0)], false );
        }

        // negaive
        let mut negative: bool = false;
        // ?num * -num
        if other.1 == true {
            // num * -num
            other.1 = false;
            if self.1 != true {
                negative = true;
            }
        } else
        // -num * +num
        if self.1 {
            negative = true;
        }

        //
        let mut result: Vec<DUInt> = Vec::new();
        let mut remainder = Self( vec![DUInt(0)], false );

        for &digit in self.0.iter().rev() {
            let mut quotientDigit: usize = 0; // todo: move to up
            remainder = remainder *Self(vec![DUInt(10)],false) +Self(vec![digit],false);

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
        // check negative zero result
        if checkZeroValue(&result) && negative {
            negative = false;
        }
        Self(result, negative)
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