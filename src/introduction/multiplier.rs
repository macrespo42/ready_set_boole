use crate::introduction::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    if a == 0 || b == 0 {
        return 0;
    }
    let mut c: u32 = a;
    let mut d: u32 = b;
    let mut result: u32 = 0;
    while d > 0 {
        if d & 1 == 1 {
            result = adder::adder(result, c);
        }
        c = c << 1;
        d = d >> 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplier_basic() {
        assert_eq!(multiplier(4, 2), 8);
        assert_eq!(multiplier(40, 20), 800);
    }

    #[test]
    fn multiplier_by_zero() {
        assert_eq!(multiplier(0, 2), 0);
        assert_eq!(multiplier(4, 0), 0);
    }

    #[test]
    fn multiplier_with_hundred() {
        assert_eq!(multiplier(400, 200), 8_0000);
    }

    #[test]
    fn multiplier_with_1k() {
        assert_eq!(multiplier(1_000, 1_000), 1_000000);
    }
}
