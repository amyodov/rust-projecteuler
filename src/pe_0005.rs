fn is_evenly_divisible(n: i64, limit: i64) -> bool {
    (1..=limit)
        .find(|x| n % x != 0)
        .is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_evenly_divisible() {
        assert!(is_evenly_divisible(6, 3));
        assert!(is_evenly_divisible(12, 3));
        assert!(!is_evenly_divisible(13, 3));
    }
}

pub fn calc() -> i64 {
    (1..)
        .find(|&x| is_evenly_divisible(x, 20))
        .expect("Something went wrong")
}
