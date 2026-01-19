pub fn add_two(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}
