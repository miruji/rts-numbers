use std::fmt;

fn isDigit(c: u8) -> bool {        // todo: use from tokenizer.rs
    c >= b'0' && c <= b'9'
}
fn isUFloat(value: &str) -> bool { // todo: add this to tokenizer.rs
    value.bytes().all(|c| isDigit(c) || c == b'.')
}

use crate::DInt::*;
use crate::DUInt::*;

// UFloat
#[derive(Clone)]
pub struct UFloat(pub Vec<DUInt>); // value
impl UFloat {
    pub fn new(value: &str) -> Self {
        if isUFloat(value) {
            //println!("1> {:?}", value.chars());
            let converted: Vec<DUInt> = value.chars().rev().map(charToDUInt).collect();
            //println!("2> {:?}", converted);
            Self(converted)
        } else {
            Self(vec![DUInt(0)])
        }
    }
}
impl fmt::Display for UFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.0.iter().rev().map(|dui| {
            if *dui == DUInt(10) {
                '.'
            } else {
                (dui.0 + b'0') as char
            }
        }).collect();
        write!(f, "{}", s)
    }
}
// UFloat + UFloat
impl std::ops::Add for UFloat {
    type Output = Self;
    fn add(mut self, mut other: Self) -> Self {
        // num1.num ? num1.num
        // len  len   len  len
        checkDigitsLength(&mut self.0, &mut other.0, false);

        //
        let mut leftDigits  = self.0.iter().cloned().peekable();
        let mut rightDigits = other.0.iter().cloned().peekable();

        let mut result: Vec<DUInt> = Vec::new();

        let mut leftDigit:  DUInt;
        let mut rightDigit: DUInt;

        let mut carry: u8 = 0;
        let mut sum:   u8;

        while leftDigits.peek().is_some() || rightDigits.peek().is_some() || carry != 0 {
            leftDigit  = leftDigits.next().unwrap_or(DUInt(0));
            rightDigit = rightDigits.next().unwrap_or(DUInt(0));
            
            // check .
            if leftDigit == DUInt(10) || rightDigit == DUInt(10) {
                result.push(DUInt(10));
                continue;
            }

            //
            sum = leftDigit.0+rightDigit.0 + carry;
            result.push( DUInt::u8(sum%10) );
            carry = sum/10;
        }

        // next result
        unnecessaryFloatResult(&mut result);
        Self(result)
    }
}
// UFloat - UFloat
impl std::ops::Sub for UFloat {
    type Output = Self;
    fn sub(mut self, mut other: Self) -> Self {
        // num1.num ? num1.num
        // len  len   len  len
        checkDigitsLength(&mut self.0, &mut other.0, true);
        //unnecessaryFloatResult(&mut other.0); // todo: and use here to self?

        // check if result will be negative
        if DUIntGreater(&self.0, &other.0, false) != std::cmp::Ordering::Greater {
            return Self( vec![DUInt(10)] );
        }

        //
        let mut leftDigits  = self.0.iter().cloned().peekable();
        let mut rightDigits = other.0.iter().cloned().peekable();

        let mut borrow: DUInt      = DUInt(0);
        let mut result: Vec<DUInt> = Vec::new();
        let mut diff:   DInt;

        while leftDigits.peek().is_some() || rightDigits.peek().is_some() {
            let leftDigit:  DUInt = leftDigits.next().unwrap_or(DUInt(0));
            let rightDigit: DUInt = rightDigits.next().unwrap_or(DUInt(0));

            // check .
            if leftDigit == DUInt(10) || rightDigit == DUInt(10) {
                result.push(DUInt(10));
                continue;
            }

            //
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
        unnecessaryFloatResult(&mut result);
        Self(result)
    }
}
// UFloat * UFloat
impl std::ops::Mul for UFloat {
    type Output = Self;
    fn mul(mut self, mut other: Self) -> Self {
        // num * 0 || 0 * num
        if checkZeroValue(&other.0) || checkZeroValue(&self.0) {
            return Self( vec![DUInt(10)] );
        }

        //
        let mut result: Vec<DUInt> = vec![DUInt(0); self.0.len() + other.0.len()];
        let mut ident: usize = 0;
        let mut afterDot: bool = false;
        for (j, &rightDigit) in other.0.iter().enumerate() {
            ident = if !afterDot {
                j
            } else {
                j-1
            };
            if rightDigit == DUInt(0) {
            //    ident -= 1;
                continue;
            }
            if rightDigit == DUInt(10) {
                afterDot = true;
                continue;
            }
            for (mut i, &leftDigit) in self.0.iter().enumerate() {
                if leftDigit == DUInt(10) {
                    continue;
                }

                let sum: u8 = rightDigit.0*leftDigit.0;
                result[ident].0 += sum;
                result[ident+1].0 += result[ident].0/10;
                result[ident].0 = result[ident].0%10;

                ident += 1;
            }
        }

        result.insert(getDigitsLengthBeforeDot(&self.0)+getDigitsLengthBeforeDot(&other.0), DUInt(10));

        // next result
        unnecessaryFloatResult(&mut result);
        Self(result)
    }
}
// UFloat / UFloat
static mut indentation: usize = 0;
impl std::ops::Div for UFloat {
    type Output = Self;
    fn div(mut self, mut other: Self) -> Self {
        // if right op = 0
        if checkZeroValue(&other.0) {
            return Self(self.0);
        }
        //
        let mut whole:  usize  = 0;
        let mut result: UFloat = UFloat(vec![DUInt(10)]);
        if DUIntGreater(&self.0, &other.0, false) == std::cmp::Ordering::Less {
            println!("[{}] [{}]",self,other);
            // multiplication size
            let mut nullsLength: usize = findDot(&other.0,false);
            println!("  nullsLength [{}], findDot [{}]",nullsLength,findDot(&self.0,false));
            let mut mode: bool = false;
            if nullsLength == 0 {
                nullsLength = findDot(&self.0,false);
                mode = true;
            // multiplication
            } else {
                other = other * UFloat::new(&format!("1{}","0".repeat(nullsLength)+".0"));
            }
            self = self * UFloat::new(&format!("1{}","0".repeat(nullsLength+1)+".0"));
//            println!("  self [{}] other [{}]",self,other);
//            println!("  DUIntGreater [{:?}]",DUIntGreater(&self.0, &other.0, false));
            // divide
            while DUIntGreater(&self.0, &other.0, false) != std::cmp::Ordering::Less {
                self = self - other.clone();
                whole += 1;
//                println!("  !!! self [{}], whole [{}]",self,whole);
            }
            // result
//            println!("  !!! self [{}]",self);
            result = UFloat::new(&whole.to_string());
            result.0.insert(0,DUInt(10));
            println!("  result: {}",result);
//            println!("! {} {}",findDot(&self.0),findDot(&other.0));
            if unsafe{ indentation < findDot(&self.0,false)+findDot(&other.0,false)+2 } && self.0 != vec![DUInt(10)] {
                unsafe{
                    println!("  mode: {}",mode);
                    println!("  indentation1: {}",indentation);
                    if mode {
                        indentation += 2;
                        println!("    result 2: {}",result);
                        result = result * UFloat::new(&format!("0.{}","0".repeat(indentation)+"1"));
                        println!("    result 2: {}",result);
                    } else {
                        indentation += 1;
                        result = result * UFloat::new(&format!("0.{}","0".repeat(indentation-1)+"1"));
                    }
                    println!("  indentation2: {}",indentation);
                }
                self = self / other;
                result = result + self.clone();
            } else {
                unsafe{
                    println!("  mode: {}",mode);
                    println!("  indentation1: {}",indentation);
                    if mode {
                        indentation += 2;
                        println!("    result 1: {}",result);
                        result = result * UFloat::new(&format!("0.{}","0".repeat(indentation-1)+"1"));
                        println!("    result 1: {}",result);
                    } else {
                        indentation += 1;
                        result = result * UFloat::new(&format!("0.{}","0".repeat(indentation-1)+"1"));
                    }
                    println!("  indentation2: {}",indentation);
                }
            }
        } else {
//            println!("[{}] [{}]",self,other);
            unsafe{ indentation = 0; }
            // divide
            while DUIntGreater(&self.0, &other.0, false) != std::cmp::Ordering::Less {
                self = self - other.clone();
                whole += 1;
//                println!("  !!! self [{}], whole [{}]",self,whole);
            }
            // result
            result = UFloat::new(&whole.to_string());
            result.0.insert(0,DUInt(10));
            // self != . then divide remainder
            if self.0 != vec![DUInt(10)] {
                self = self / other;
                result = result + self.clone();
            }
        }
        result
    }
}
// find float dot
fn findDot(float: &Vec<DUInt>, rev: bool) -> usize {
    let mut dotIndex: usize = 0;
    if rev == false {
        for (index, item) in float.iter().enumerate() {
            if *item == DUInt(10) {
                dotIndex = index;
                break;
            }
        }
    } else {
        for (index, item) in float.iter().rev().enumerate() {
            if *item == DUInt(10) {
                dotIndex = index;
                break;
            }
        }
    }
    return dotIndex;
}
// check value = 0
fn checkZeroValue(result: &Vec<DUInt>) -> bool { // todo: merge this
    *result == vec![DUInt(0)] || *result == vec![DUInt(10)]
}
// get digits length before .
// e: 0.00 -> 2
fn getDigitsLengthBeforeDot(vec: &[DUInt]) -> usize {
    vec.iter().take_while(|&&x| x.0 != 10).count()
}
// remove unnecessary result
fn unnecessaryFloatResult(result: &mut Vec<DUInt>) { // todo: merge this FLOAT ONLY
    // remove some 0
    while let Some(&DUInt(0)) = result.last() {
        result.pop();
    }
    while let Some(&DUInt(0)) = result.first() {
        result.remove(0);
    }
    // empty = 0
    if result.is_empty() {
        *result = vec![DUInt(10)]
    }
}