use std::collections::HashSet;

// Implement an algorithm to determine if a string has all unique characters
// What if you can not use additional data structures?
pub fn solve(str: &String) -> bool {
    let mut unique_set = HashSet::new();
    let vec_iter = str.as_bytes();
    for c in vec_iter {
        unique_set.insert(c);
    }
    unique_set.len() == str.len()
}

#[cfg(test)]
mod test {
    use chapter1::ex11::solve;

    #[test]
    fn test_result_ok() {
        assert_eq!(true, solve(&"True".to_string()));
        assert_eq!(true, solve(&"今日は".to_string()));
    }

    #[test]
    fn test_result_fail() {
        assert_eq!(false, solve(&"Duuplicate".to_string()));
        assert_eq!(false, solve(&"今日はは".to_string()));
    }

}
