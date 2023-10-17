use std::convert::identity;

fn digits_of_number(n: u64) -> impl Iterator<Item=u8> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>()
        .into_iter()
}

fn champernowne_digits() -> impl Iterator<Item=u8> {
    (1..).map(digits_of_number).flat_map(identity)
}

pub fn calc() -> u64 {
    champernowne_digits()
        .enumerate()
        .map(|(i, v)| (i + 1, v as u64))
        .take_while(|(i, _)| *i <= 1_000_000)
        .filter(|(i, _)| {
            false ||
                *i == 1 ||
                *i == 10 ||
                *i == 100 ||
                *i == 1_000 ||
                *i == 10_000 ||
                *i == 100_000 ||
                *i == 1_000_000
        })
        .map(|(_, v)| v)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    fn iterator_as_list<T>(ii: impl Iterator<Item=T>) -> String
        where T: std::fmt::Display + std::fmt::Debug {
        ii.map(|x| x.to_string()).join(",")
    }

    #[test]
    fn test_digits_of_number() {
        assert_eq!(iterator_as_list(digits_of_number(239487239842234)),
                   "2,3,9,4,8,7,2,3,9,8,4,2,2,3,4")
    }

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
