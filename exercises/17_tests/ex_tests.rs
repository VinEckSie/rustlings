fn main() {
    println!("hey tests");
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn validate_addition() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn fail() {
        panic!("code panicked")
    }
}
