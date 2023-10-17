pub fn digits_of_number(n: u64) -> impl Iterator<Item=u8> {
    let mut n_op = n;
    let est_digits = n.ilog10() + 1;
    let mut result: Vec<u8> = Vec::with_capacity(est_digits as usize);

    while n_op > 0 {
        let digit = n_op % 10;
        n_op = n_op / 10;
        result.push(digit as u8);
    }

    result.into_iter().rev()
}


#[cfg(test)]
mod tests {
    use super::*;
    use pe_itertools::iterator_as_list;

    #[test]
    fn test_digits_of_number() {
        assert_eq!(iterator_as_list(digits_of_number(239487239842234)),
                   "2,3,9,4,8,7,2,3,9,8,4,2,2,3,4");
    }
}
