use std::fmt;

// DUInt
// u8 | >= 0 | <= 9 | == 10 => .
#[derive(Clone, PartialEq, PartialOrd, Copy)]
pub struct DUInt (pub u8);
impl DUInt {
    pub fn isize(value: isize) -> Self {
        if value <= 0 {
            Self(0)
        } else
        if value >= 9 {
            Self(9)
        } else {
            Self(value as u8)
        }
    }
    pub fn u8(value: u8) -> Self {
        if value <= 0 {
            Self(0)
        } else
        if value >= 9 {
            Self(9)
        } else {
            Self(value)
        }
    }
}
impl fmt::Display for DUInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl fmt::Debug for DUInt { // todo: remove this ?
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
// +
impl std::ops::Add for DUInt {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        DUInt( (self.0 + other.0) % 10 ) // todo: ?
    }
}
// %
impl std::ops::Rem for DUInt {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        DUInt( self.0 % rhs.0 )
    }
}
// /
impl std::ops::Div for DUInt {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        DUInt( self.0 / rhs.0 )
    }
}
// +=
impl std::ops::AddAssign for DUInt {
    fn add_assign(&mut self, other: DUInt) {
        self.0 += other.0;
    }
}
// char to DUInt
pub fn charToDUInt(c: char) -> DUInt { // rename (toDUInt)
    match c {
        '0'..='9' => DUInt(c as u8 - b'0'),
        '.' => DUInt(10),
        _ => DUInt(0),
    }
}
// check vec1 greater = true ?
// todo: separate version for int
pub fn DUIntGreater(left: &Vec<DUInt>, right: &Vec<DUInt>, int: bool) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    // check end -> begin
    // int
    if int {
        let maxLen: usize = left.len().max(right.len());
        for i in (0..maxLen).rev() {
            let leftElem:  u8 = if i < left.len() { left[i].0 } else { 0 };
            let rightElem: u8 = if i < right.len() { right[i].0 } else { 0 };
            // skip . to check next digits
            if leftElem == 10 || rightElem == 10 {
                continue;
            }
            //
            if leftElem > rightElem {
                return Ordering::Greater;
            } else 
            if leftElem < rightElem {
                return Ordering::Less;
            } else 
            if leftElem == rightElem {
                return Ordering::Equal;
            }
        }
    } else {
    // float
        //
        let mut leftBuffer = left.clone();
        let mut rightBuffer = right.clone();
        checkDigitsLength(&mut leftBuffer, &mut rightBuffer, true);
        //
        let leftFindedDot = findDot(&leftBuffer);
        let leftLeftPart  = &leftBuffer[..leftFindedDot].to_vec();
        let leftRightPart = &leftBuffer[leftFindedDot +1..].to_vec();
        //
        let rightFindedDot = findDot(&rightBuffer);
        let rightLeftPart  = &rightBuffer[..rightFindedDot].to_vec();
        let rightRightPart = &rightBuffer[rightFindedDot +1..].to_vec();
        // left part
        let mut maxLen1: usize = leftLeftPart.len().max(rightLeftPart.len());
        let mut leftResult: std::cmp::Ordering = Ordering::Equal;
        for i in 0..maxLen1 {
            let mut leftElem:  u8 = if i < leftLeftPart.len() { leftLeftPart[leftLeftPart.len()-i-1].0 } else { 0 };
            let mut rightElem: u8 = if i < rightLeftPart.len() { rightLeftPart[rightLeftPart.len()-i-1].0 } else { 0 };
            if leftElem > rightElem {
                leftResult = Ordering::Greater;
                break;
            } else 
            if leftElem < rightElem {
                leftResult = Ordering::Less;
                break;
            } else 
            if leftElem == rightElem {
                leftResult = Ordering::Equal;
                continue;
            }
        }
        // right part
        maxLen1 = leftRightPart.len().max(rightRightPart.len());
        let mut rightResult: std::cmp::Ordering = Ordering::Equal;
        for i in 0..maxLen1 {
            let mut leftElem:  u8 = if i < leftRightPart.len() { leftRightPart[leftRightPart.len()-i-1].0 } else { 0 };
            let mut rightElem: u8 = if i < rightRightPart.len() { rightRightPart[rightRightPart.len()-i-1].0 } else { 0 };
            if leftElem > rightElem {
                rightResult = Ordering::Greater;
                break;
            } else 
            if leftElem < rightElem {
                rightResult = Ordering::Less;
                break;
            } else 
            if leftElem == rightElem {
                rightResult = Ordering::Equal;
                continue;
            }
        }
        // check right part
        if rightResult == Ordering::Greater {
            return Ordering::Greater;
        } else
        if rightResult == Ordering::Less {
            return Ordering::Less;
        } else
        if rightResult == Ordering::Equal {
            // check left part
            if leftResult == Ordering::Greater {
                return Ordering::Greater;
            } else
            if leftResult == Ordering::Equal {
                return Ordering::Equal;
            } else {
                return Ordering::Less;
            }
        }
    }
    // todo: rewrite this down part
    // all equal
    if left.len() > right.len() {
        Ordering::Greater
    } else
    if left.len() < right.len() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}
// find float dot
fn findDot(float: &Vec<DUInt>) -> usize {
    let mut dotIndex: usize = 0;
    for (index, item) in float.iter().enumerate() {
        if *item == DUInt(10) {
            dotIndex = index;
            break;
        }
    }
    return dotIndex;
}
// adding the necessary zeros
// check num1.num ? num2.num 
//       len  len   len  len
// **    10  . 0   + 0  . 01 = 10.01
//       l=2  l=1   l=1  l=2
pub fn checkDigitsLength(left: &mut Vec<DUInt>, right: &mut Vec<DUInt>, swapBack: bool){
    std::mem::swap(left, right);

    let leftFindedDot  = findDot(left);
    let rightFindedDot = findDot(right);

    // before .
    let leftPartBeforeLength  = leftFindedDot;
    let rightPartBeforeLength = rightFindedDot;
    let insertLeftBeforeLength  = rightPartBeforeLength.saturating_sub(leftPartBeforeLength);
    let insertRightBeforeLength = leftPartBeforeLength.saturating_sub(rightPartBeforeLength);

    // after .
    let leftPartAfterLength  = left.len()  - leftFindedDot  - 1;
    let rightPartAfterLength = right.len() - rightFindedDot - 1;
    let insertLeftAfterLength  = rightPartAfterLength.saturating_sub(leftPartAfterLength);
    let insertRightAfterLength = leftPartAfterLength.saturating_sub(rightPartAfterLength);

    // add left
    left.resize_with(left.len()   + insertLeftAfterLength, || DUInt(0));
    left.splice(0..0,  std::iter::repeat(DUInt(0)).take(insertLeftBeforeLength));

    // add right
    right.resize_with(right.len() + insertRightAfterLength, || DUInt(0));
    right.splice(0..0, std::iter::repeat(DUInt(0)).take(insertRightBeforeLength));

    if swapBack {
        std::mem::swap(left, right);
    }
}
