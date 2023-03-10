use std::cmp::PartialOrd;
use std::error::Error;
use std::fmt;

use num::traits::Num;
use num_traits::{NumCast, NumOps};

#[derive(Debug, PartialEq)]
pub struct ConversionError {
    details: String,
}

impl ConversionError {
    fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ConversionError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Clone)]
pub struct Digits<T> {
    values: Vec<T>,
    index: usize,
}

impl<T> From<T> for Digits<T>
where
    T: Num + NumCast + PartialOrd + Copy,
{
    fn from(i: T) -> Self {
        match digits_from_int(i) {
            Ok(values) => Self { values, index: 0 },
            Err(err) => {
                panic!("{}", err)
            }
        }
    }
}

impl<T> std::ops::Deref for Digits<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<T> Iterator for Digits<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.values.len() {
            return None;
        }
        self.index += 1;
        Some(self.values[self.index - 1])
    }
}

fn digits_from_int<T>(n: T) -> Result<Vec<T>, ConversionError>
where
    T: Num + NumCast + PartialOrd + Copy,
{
    let zero = T::from(0).unwrap();
    let ten = T::from(10).unwrap();

    match n {
        _ if n >= zero => {
            let mut rem = n;
            let mut v = Vec::new();

            while (rem / ten) > zero {
                let last = rem % ten;
                rem = rem / ten;
                v.insert(0, last);
            }
            v.insert(0, rem);

            Ok(v)
        }
        _ => Err(ConversionError::new(
            "unable to convert from negative integer to digits",
        )),
    }
}

fn int_from_digits<T>(v: Vec<T>) -> T
where
    T: Num + NumCast + NumOps + Copy,
{
    let mut number = T::from(0).unwrap();
    let ten = T::from(10).unwrap();
    for (idx, mut digit) in v.into_iter().rev().enumerate() {
        for _ in 0..idx {
            digit = digit * ten;
        }
        number = number + digit;
    }
    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_struct_works() {
        let mut digits = Digits::from(42);
        assert_eq!(digits.next(), Some(4));
        assert_eq!(digits.next(), Some(2));
        assert_eq!(digits.next(), None);
    }

    #[test]
    fn iterator_adapters_work() {
        let digits = Digits::from(42);
        assert_eq!(digits.len(), 2);

        let digits = Digits::from(369);
        assert_eq!(digits.count(), 3);

        let mut digits = Digits::from(42).cycle();
        assert_eq!(digits.next(), Some(4));
        assert_eq!(digits.next(), Some(2));
        assert_eq!(digits.next(), Some(4));
        assert_eq!(digits.next(), Some(2));
        assert_eq!(digits.next(), Some(4));
        assert_eq!(digits.next(), Some(2));

        let mut digits = Digits::from(369).enumerate();
        assert_eq!(digits.next(), Some((0, 3)));
        assert_eq!(digits.next(), Some((1, 6)));
        assert_eq!(digits.next(), Some((2, 9)));
        assert_eq!(digits.next(), None);

        let digits = Digits::from(369);
        assert_eq!(digits.fold(0, |acc, x| acc + x), 18);
    }

    #[test]
    fn contains_works() {
        let digits = Digits::from(369);
        assert!(digits.contains(&3));
        assert_eq!(digits.contains(&4), false);
    }

    #[test]
    fn digits_from_works() {
        assert_eq!(digits_from_int(42), Ok(vec![4, 2]));
    }

    #[test]
    fn digits_from_works_with_zero() {
        assert_eq!(digits_from_int(0), Ok(vec![0]));
    }

    #[test]
    fn digits_from_works_with_zeros() {
        assert_eq!(digits_from_int(00), Ok(vec![0]));
    }

    #[test]
    fn digits_from_works_with_u8_max() {
        assert_eq!(digits_from_int(u8::MAX), Ok(vec![2, 5, 5]));
    }

    #[test]
    fn digits_from_works_with_u16_max() {
        assert_eq!(digits_from_int(u16::MAX), Ok(vec![6, 5, 5, 3, 5]));
    }

    #[test]
    fn digits_from_works_with_u32_max() {
        assert_eq!(
            digits_from_int(u32::MAX),
            Ok(vec![4, 2, 9, 4, 9, 6, 7, 2, 9, 5])
        );
    }

    #[test]
    fn digits_from_works_with_u64_max() {
        assert_eq!(
            digits_from_int(u64::MAX),
            Ok(vec![
                1, 8, 4, 4, 6, 7, 4, 4, 0, 7, 3, 7, 0, 9, 5, 5, 1, 6, 1, 5
            ])
        );
    }

    #[test]
    fn digits_from_throws_error_with_negative_number() {
        assert_eq!(
            digits_from_int(-42),
            Err(ConversionError {
                details: "unable to convert from negative integer to digits".to_string()
            })
        );
    }

    #[test]
    fn from_digits_works() {
        let v = vec![4, 2];
        assert_eq!(int_from_digits(v), 42);
    }

    #[test]
    fn from_digits_works_with_zero() {
        assert_eq!(int_from_digits(vec![0]), 0);
    }

    #[test]
    fn from_digits_works_with_u8_max() {
        assert_eq!(int_from_digits(vec![2, 5, 5]), u8::MAX);
    }

    #[test]
    fn from_digits_works_with_u16_max() {
        assert_eq!(int_from_digits(vec![6, 5, 5, 3, 5]), u16::MAX);
    }

    #[test]
    fn from_digits_works_with_u32_max() {
        assert_eq!(
            int_from_digits(vec![4, 2, 9, 4, 9, 6, 7, 2, 9, 5]),
            u32::MAX
        );
    }

    #[test]
    fn from_digits_works_with_u64_max() {
        assert_eq!(
            int_from_digits(vec![
                1, 8, 4, 4, 6, 7, 4, 4, 0, 7, 3, 7, 0, 9, 5, 5, 1, 6, 1, 5
            ]),
            u64::MAX
        );
    }
}
