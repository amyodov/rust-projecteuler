use std::ops::{Div, Rem};
use num::{FromPrimitive, Integer, ToPrimitive};

/// Make an Iterator with the digits of number.
///
/// Generic function; supports both `u64`/`u128` and `BigUint`.
pub fn digits<T>(n: T) -> impl Iterator<Item=u8>
    where T: Rem<T, Output=T> + Div<T, Output=T> + FromPrimitive + ToPrimitive + Integer
{
    let zero: T = FromPrimitive::from_u64(0).unwrap();
    let ten: T = FromPrimitive::from_u64(10).unwrap();

    let mut n: T = n;
    let est_digits = 10;
    let mut result: Vec<u8> = Vec::with_capacity(est_digits as usize);
    let mut digit: T;

    while n > zero {
        (n, digit) = n.div_rem(&ten);
        result.push(digit.to_u8().unwrap());
    }

    result.into_iter().rev()
}

/// How many digits are in a number?
pub fn digits_count<T>(n: T) -> usize
    where T: Rem<T, Output=T> + Div<T, Output=T> + FromPrimitive + ToPrimitive + Integer {
    digits(n).count()
}


#[cfg(test)]
mod tests {
    use super::*;
    use num::bigint::BigUint;
    use pe_itertools::iterator_as_list;

    #[test]
    fn test_digits() {
        assert_eq!(iterator_as_list(digits(23948723)),
                   "2,3,9,4,8,7,2,3");
        assert_eq!(iterator_as_list(digits(239487239842234u64)),
                   "2,3,9,4,8,7,2,3,9,8,4,2,2,3,4");
        assert_eq!(iterator_as_list(digits(239487239842234u128)),
                   "2,3,9,4,8,7,2,3,9,8,4,2,2,3,4");
        assert_eq!(iterator_as_list(digits(BigUint::from(239487239842234u64))),
                   "2,3,9,4,8,7,2,3,9,8,4,2,2,3,4");
    }

    #[test]
    fn test_digits_count() {
        assert_eq!(digits_count(239487239842234u128),
                   15);
        assert_eq!(digits_count(BigUint::from(239487239842234u64)),
                   15);
    }
}
