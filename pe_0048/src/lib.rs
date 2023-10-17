use num_bigint::BigUint;

fn last_digits(n: BigUint, n_digits: u8) -> u64 {
    let pow_of_10 = BigUint::from(10u8).pow(n_digits as u32);
    (n % pow_of_10).try_into().unwrap()
}

fn sum_facts(limit: usize) -> BigUint {
    (1..=limit)
        .map(|i| BigUint::from(i).pow(i as u32))
        .sum()
}

pub fn calc() -> u64 {
    last_digits(sum_facts(1000), 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_last_digits() {
        assert_eq!(last_digits(BigUint::from_str("1234567890").unwrap(), 3),
                   890);
        assert_eq!(last_digits(BigUint::from_str("12345678901234567890").unwrap(), 10),
                   1234567890);
    }

    #[test]
    fn test_sum_facts() {
        assert_eq!(sum_facts(10), BigUint::from(10405071317u64));
    }

    #[test]
    fn test_calc() {
        assert_eq!(calc(), 9110846700);
    }
}
