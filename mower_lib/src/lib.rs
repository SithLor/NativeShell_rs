//marco to get size of &str
macro_rules! size_of_str {
    ($s:expr) => {
        $s.len()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
