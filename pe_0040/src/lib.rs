use std::convert::identity;

use pe_digits::digits;


fn champernowne_digits() -> impl Iterator<Item=u8> {
    (1..).map(digits).flat_map(identity)
}

pub fn calc() -> u64 {
    champernowne_digits()
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .take_while(|(i, _)| *i <= 1_000_000)
        .filter(|(i, _)| match *i {
            1 | 10 | 100 | 1_000 | 10_000 | 100_000 | 1_000_000 => true,
            _ => false
        })
        .map(|(_, v)| v as u64)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pe_itertools::iterator_as_list;

    #[test]
    fn test_champernowne_digits() {
        let first_15 = champernowne_digits()
            .enumerate()
            .take_while(|(i, _)| *i < 15)
            .map(|(_, v)| v);

        assert_eq!(iterator_as_list(first_15),
                   "1,2,3,4,5,6,7,8,9,1,0,1,1,1,2");
    }

    #[test]
    fn test_calc() {
        assert_eq!(calc(), 210);
    }
}
