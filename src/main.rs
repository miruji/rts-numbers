#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

mod DInt;
mod DUInt;

mod Int;
mod UInt;
//mod Float;
mod UFloat;

fn main() {
    use crate::Int::*;
    use crate::UInt::*;
    //use crate::Float::*;
    use crate::UFloat::*;

    //use crate::DInt::*;
    //use crate::DUInt::*;

    //let d1: DInt = DInt::new(-100);
    //println!("DInt: {}",d1.0);

    //let d2: DUInt = DUInt::new(100);
    //println!("DUInt: {}",d2.0);

    //let aa: u128 = 100;
    //let bb: u128 = 110;

    let a = UFloat::new("4.14"); // 4.14
    let b = UFloat::new("10."); // 100.
    // 4.14 / 10 && 4.14 / 100 && 4.14 / 1 <== ошибка со сдвигом
    // 3.14 / 1000 = 0.00314
    // 12.0 / 3.31 = 3.6253776435045317

    let c = a.clone() + b.clone();
    let d = a.clone() - b.clone();
    let e = a.clone() * b.clone();
    let f = a.clone() / b.clone();

    println!("+\n  {} +\n  {} =\n  {}", a, b, c);
    println!("-\n  {} -\n  {} =\n  {}", a, b, d);
    println!("*\n  {} *\n  {} =\n  {}", a, b, e);
    println!("/\n  {} /\n  {} =\n  {}", a, b, f);

    //

    let a = BigInt::from(1000000000000000000); 
    let b = BigInt::from(1000000000000000000);
    let sum = a + b; 
    println!("Sum: {}", sum);
}

/*
todo: DigitType test file

+
    UInt
    100 + 100 = 100

    Int
     100 +  100 = 100
     100 + -100 = 0
    -100 +  100 = 0
    -100 + -100 = -200

    UFloat
    .1 + 0.01 = .11

    Float

-
    UInt
    100 - 100 = 0
    100 - 101 = 0

    Int
     100 -  101 = -1   | -
     100 - -100 =  200 | +
    -100 -  100 = -200 | -
    -100 - -100 =  0   | +

    UFloat
    .21 - .20 = .01
    .21 - .21 = 0

    Float

*
    UInt
    10 * 10 = 100
    10 * 0  = 0
    10 * 1  = 0

    Int
     10 *  10 =  100 | +
     10 * -10 = -100 | -
    -10 *  10 = -100 | -
    -10 * -10 =  100 | +
    -10 *  0  =  0
    -10 *  1  = -10

    UFloat

    Float

/
    UInt
    10 / 10 = 1
    10 / 0  = 10
    10 / 1  = 10

    Int
     10 /  10 =  1 | +
     10 / -10 = -1 | -
    -10 /  10 = -1 | -
    -10 / -10 =  1 | +
    -10 /  0  = -10
    -10 /  1  = -10

    UFloat

    Float

*/