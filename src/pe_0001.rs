fn items() -> impl Iterator<Item=i64> {
    (1..1000).filter(|x| x % 5 == 0 || x % 3 == 0)
}

pub fn calc() -> i64 {
    items().sum()
}
