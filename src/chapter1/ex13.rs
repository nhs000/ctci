//Design an algorithm and write code to remove the duplicate characters in a string without using any additional bufer
//NOTE: One or two additional variables are fine
//An extra copy of the array is not 

fn solve(str: String) -> String {
    str.chars().rev().collect::<String>()
}

#[cfg(test)]
mod test {
    use chapter1::ex13::solve;

    #[test]
    fn test_result_ok() {
        assert_eq!("cba", solve("abc".to_string()));
        assert_eq!("はちにんこ", solve("こんにちは".to_string()));
    }
}

