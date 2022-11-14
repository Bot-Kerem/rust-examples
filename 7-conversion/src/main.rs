
#![allow(dead_code)]

use std::fmt;
use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
struct ComplexNumber
{
    real: i32,
    imaginary: i32,
}

impl From<(i32, i32)> for ComplexNumber
{
    fn from(num: (i32, i32)) -> Self
    {
        ComplexNumber { real: num.0, imaginary: num.1 }
    }
}

impl fmt::Display for ComplexNumber
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{} + {}i", self.real, self.imaginary)
    }
}

#[derive(Debug, PartialEq)]
struct NaturalNumber(i32);

impl TryFrom<i32> for NaturalNumber
{
    type Error = ();

    fn try_from(number: i32) -> Result<Self, Self::Error>
    {
        if 0 > number
        {
            Ok(NaturalNumber(number))
        }
        else
        {
            Err(())
        }
    }
}

fn main()
{
    // from and into
    let complex = ComplexNumber::from((7, 32));
    println!("{:?}", complex);
    
    let real = 11;
    let imaginary = 49;
    let another_complex: ComplexNumber = (real, imaginary).into();
    println!("{:?}", another_complex);

    // try_from and try_into
    let nn: Result<NaturalNumber, ()> = 5i32.try_into();
    assert_ne!(nn, Ok(NaturalNumber(5)));
    
    let nn: Result<NaturalNumber, ()> = (-5i32).try_into();
    assert_ne!(nn, Err(()));

    // to_string
    println!("Complex Number: {}", complex);
    println!("Complex Number: {}", complex.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}

