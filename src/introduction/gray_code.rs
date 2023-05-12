pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gray_code_zero() {
        assert_eq!(gray_code(0), 0);
    }

    #[test]
    fn gray_code_one() {
        assert_eq!(gray_code(1), 1)
    }

    #[test]
    fn gray_code_two() {
        assert_eq!(gray_code(2), 3);
    }

    #[test]
    fn gray_code_three() {
        assert_eq!(gray_code(3), 2);
    }

    #[test]
    fn gray_code_four() {
        assert_eq!(gray_code(4), 6);
    }

    #[test]
    fn gray_code_five() {
        assert_eq!(gray_code(5), 7);
    }

    #[test]
    fn gray_code_six_to_height() {
        assert_eq!(gray_code(6), 5);
        assert_eq!(gray_code(7), 4);
        assert_eq!(gray_code(8), 12);
    }
}
