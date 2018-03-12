/*

    Internal implementation of module functions.

*/

pub fn sum_two_longs(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sum_two_dwords(a: u32, b: u32) -> u32 {
    a + b
}

/*

	Unit tests for the functions above.
	You can run them via `cargo test`.

*/

#[cfg(test)]
mod tests {
    use super::*;	// Import functions from outer scope

    #[test]
    fn test_sum_two_longs() {
        assert_eq!(sum_two_longs(-3, 7), 4);
    }

    #[test]
    fn test_sum_two_dwords() {
        assert_eq!(sum_two_dwords(2000000000, 1000000000), 3000000000);
    }
}
