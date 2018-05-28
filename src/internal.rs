/*

    Internal implementation of module functions.

*/
pub fn sum_two_integers(a: i16, b: i16) -> i16 {
    a + b
}

pub fn sum_two_longs(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sum_two_quads(a: i64, b: i64) -> i64 {
    a + b
}

pub fn sum_two_bytes(a: u8, b: u8) -> u8 {
    a + b
}

pub fn sum_two_words(a: u16, b: u16) -> u16 {
    a + b
}

pub fn sum_two_dwords(a: u32, b: u32) -> u32 {
    a + b
}

pub fn sum_two_singles(a: f32, b: f32) -> f32 {
    a + b
}

pub fn sum_two_doubles(a: f64, b: f64) -> f64 {
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
    fn test_sum_two_integers() {
        assert_eq!(sum_two_integers(-10000, 20000), 10000);
    }

    #[test]
    fn test_sum_two_longs() {
        assert_eq!(sum_two_longs(-600000, 700000), 100000);
    }

    #[test]
    fn test_sum_two_quads() {
        assert_eq!(sum_two_quads(4000000000, 4000000000), 8000000000);
    }    

    #[test]
    fn test_sum_two_bytes() {
        assert_eq!(sum_two_bytes(64, 128), 192);
    }

    #[test]
    fn test_sum_two_words() {
        assert_eq!(sum_two_words(32000, 32000), 64000);
    }

    #[test]
    fn test_sum_two_dwords() {
        assert_eq!(sum_two_dwords(2000000000, 1000000000), 3000000000);
    }

    #[test]
    fn test_sum_two_singles() {
        assert_eq!(sum_two_singles(1.111111, 1.111111), 2.222222);
    }

    #[test]
    fn test_sum_two_doubles() {
        assert_eq!(sum_two_doubles(2.2222222222222222, 1.1111111111111111), 3.3333333333333333);
    }
}
