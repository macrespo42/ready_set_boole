use crate::adder::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    if a == 0 || b == 0 {
        return 0;
    }
    let mut c: u32 = a;
    let mut d: u32 = b;
    let mut result: u32 = 0;
    while d > 0 {
        if d % 2 != 0 {
            result = adder(result, c);
        }
        c = c << 1;
        d = d >> 1;
    }
    result
}
