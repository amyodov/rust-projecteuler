const CALC_LIMIT: u64 = 28124;

/// Generate the iterator with proper divisors of a number.
fn proper_divisors(n: u64) -> impl Iterator<Item=u64> {
    (1..n).filter(move |i| n % i == 0)
}

/// Calculates whether the number is abundant.
///
/// A number `n` is called _deficient_ if the sum of its proper divisors is less than `n`,
///  and it is called _abundant_ if this sum exceeds `n`.
fn is_abundant(n: u64) -> bool {
    proper_divisors(n).sum::<u64>() > n
}

/// Generate all abundant numbers up to (including) limit.
fn gen_enough_abundant_numbers(limit: u64) -> impl Iterator<Item=u64> {
    (1..=limit).filter(|&i| is_abundant(i))
}

/// Assuming the global limit in `CALC_LIMIT` (of calculate), generate the numbers
/// which cannot be represented as a sum of 2 abundant numbers
fn gen_numbers_not_a_sum_of_2_abundant_numbers() -> impl Iterator<Item=u64> {
    let abundants: Vec<u64> = gen_enough_abundant_numbers(CALC_LIMIT).collect();

    const IS_SUM_OF_2_AB_SIZE: usize = (CALC_LIMIT * 2 + 1) as usize;
    // This array contains `true` if a number at the index (starting from 0)
    // is a sum of 2 abundant numbers, `false` otherwise.
    let mut is_sum_of_2_ab: [bool; IS_SUM_OF_2_AB_SIZE] = [false; IS_SUM_OF_2_AB_SIZE];

    for i in &abundants {
        for j in &abundants {
            let ind = i + j;
            is_sum_of_2_ab[ind as usize] = true;
        }
    }
    is_sum_of_2_ab
        .into_iter()
        .enumerate()
        .take_while(|(i, _b)| *i <= CALC_LIMIT as usize)
        .filter_map(|(i, b)| if b { None } else { Some(i as u64) })
}

pub fn calc() -> u64 {
    let nums = gen_numbers_not_a_sum_of_2_abundant_numbers();
    nums.sum::<u64>()
}


#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    fn iterator_as_list<T: ToString>(ii: impl Iterator<Item=u64>) -> String {
        ii.map(|x| x.to_string()).join(",")
    }

    #[test]
    fn test_proper_divisors() {
        assert_eq!(iterator_as_list(proper_divisors(0)),
                   "");
        assert_eq!(iterator_as_list(proper_divisors(1)),
                   "");
        assert_eq!(iterator_as_list(proper_divisors(3)),
                   "1");
        assert_eq!(iterator_as_list(proper_divisors(4)),
                   "1,2");
        assert_eq!(iterator_as_list(proper_divisors(28)),
                   "1,2,4,7,14");
        assert_eq!(iterator_as_list(proper_divisors(60)),
                   "1,2,3,4,5,6,10,12,15,20,30");
    }

    #[test]
    fn test_gen_enough_abundant_numbers() {
        // sequence [A005101](https://oeis.org/A005101) in the OEIS
        assert_eq!(iterator_as_list(gen_enough_abundant_numbers(120)),
                   "12,18,20,24,30,36,40,42,48,54,56,60,66,70,72,78,80,84,88,90,96,100,102,104,108,112,114,120");
    }

    #[test]
    fn test_is_abundant() {
        const SOME_ABUNDANT_NUMBERS: [u64; 28] = [
            12, 18, 20, 24, 30, 36, 40, 42, 48, 54, 56, 60, 66, 70, 72, 78, 80, 84, 88, 90, 96, 100,
            102, 104, 108, 112, 114, 120
        ];
        for n in 1..120u64 {
            if SOME_ABUNDANT_NUMBERS.contains(&n) {
                assert!(is_abundant(n), "{n} should be abundant");
            } else {
                assert!(!is_abundant(n), "{n} should be non-abundant");
            }
        }
    }


    #[test]
    fn test_calc() {
        assert_eq!(calc(), 4179871);
    }
}
