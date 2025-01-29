use std::fmt;

// DInt
// i8 | >= -9 | <= 9
#[derive(Clone)]
pub struct DInt (pub i8);
impl DInt {
    pub fn new(value: isize) -> Self {
        if value <= -9 {
            Self(-9)
        } else
        if value >= 9 {
            Self(9)
        } else {
            Self(value as i8)
        }
    }
}
impl fmt::Display for DInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl fmt::Debug for DInt { // todo: remove this ?
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
// -
impl std::ops::Sub for DInt {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        DInt::new( (self.0 - other.0).into() )
    }
}