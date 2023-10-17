use itertools::Itertools;

/// For an iterator of printable items, generate the string with its contents.
pub fn iterator_as_list<T>(ii: impl Iterator<Item=T>) -> String
    where T: std::fmt::Display + std::fmt::Debug {
    ii.map(|x| x.to_string()).join(",")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_as_list() {
        assert_eq!(iterator_as_list([5, 6, 7, 1, 2, 3].iter()), "5,6,7,1,2,3");
    }
}
