use fibext::Fibonacci;
use num::BigUint;
use pe_digits::digits_count;

const TARGET_LENGTH: usize = 1000;

/// Find the first index of Fibonacci number, which length is at least `n` digits.
/// The indices start from 0.
fn index_of_fibo_containing_n_digits(n: usize) -> usize {
    let fibo: Fibonacci<BigUint> = Fibonacci::new();
    fibo.skip(1)
        .enumerate()
        .find(|(_, v)| digits_count(v.clone()) == n)
        .map(|(i, _)| i)
        .unwrap()
}

pub fn calc() -> u64 {
    (index_of_fibo_containing_n_digits(TARGET_LENGTH) + 1) as u64
}

#[cfg(test)]
mod tests {
    use num::bigint::BigUint;
    use pe_digits::digits;
    use pe_itertools::iterator_as_list;
    use super::*;

    #[test]
    fn test_fibext() {
        let twelveth_fibo_u64: u64 = Fibonacci::new()
            .skip(1)
            .enumerate()
            .find(|(i, _)| *i == 12 - 1)
            .map(|(_, v)| v)
            .unwrap();
        assert_eq!(twelveth_fibo_u64,
                   144u64);

        let twelveth_fibo_biguint: BigUint = Fibonacci::new()
            .skip(1)
            .enumerate()
            .find(|(i, _)| *i == 12 - 1)
            .map(|(_, v)| v)
            .unwrap();
        assert_eq!(twelveth_fibo_biguint,
                   BigUint::from(144u64));

        assert_eq!(iterator_as_list(digits(twelveth_fibo_biguint)),
                   "1,4,4");
    }

    #[test]
    fn test_index_of_fibo_containing_n_digits() {
        assert_eq!(index_of_fibo_containing_n_digits(3), 12 - 1);
    }

    #[test]
    fn test_calc() {
        assert_eq!(calc(), 4782);
    }
}
