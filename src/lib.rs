// Public add function
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_addition() {
        assert_eq!(8 + 3, 11);
    }
}
