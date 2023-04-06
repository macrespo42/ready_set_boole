pub mod introduction;

use crate::introduction::*;

#[cfg(test)]
mod introduction_test {
    use super::*;

    #[test]
    fn adder_tests() {
       assert_eq!(adder::adder(4, 2), 6);
       assert_eq!(adder::adder(0, 0), 0);
       assert_eq!(adder::adder(4, 0), 4);
       assert_eq!(adder::adder(0, 2), 2);
       assert_eq!(adder::adder(4000, 2000), 6000);
       assert_eq!(adder::adder(40000, 20000), 60000);
    }

    #[test]
    fn multiplier_tests() {
        assert_eq!(multiplier::multiplier(4, 2), 8);
        assert_eq!(multiplier::multiplier(0, 2), 0);
        assert_eq!(multiplier::multiplier(4, 0), 0);
        assert_eq!(multiplier::multiplier(40, 20), 800);
        assert_eq!(multiplier::multiplier(400, 200), 80000);
    }

    #[test]
    fn gray_code_tests() {
        assert_eq!(gray_code::gray_code(0), 0);
        assert_eq!(gray_code::gray_code(1), 1);
        assert_eq!(gray_code::gray_code(2), 3);
        assert_eq!(gray_code::gray_code(3), 2);
        assert_eq!(gray_code::gray_code(4), 6);
        assert_eq!(gray_code::gray_code(5), 7);
        assert_eq!(gray_code::gray_code(6), 5);
        assert_eq!(gray_code::gray_code(7), 4);
        assert_eq!(gray_code::gray_code(8), 12);
    }

    #[test]
    fn eval_formula_tests() {
        assert_eq!(boolean_evaluation::eval_formula("10&"), false);
        assert_eq!(boolean_evaluation::eval_formula("10|"), true);
        assert_eq!(boolean_evaluation::eval_formula("11>"), true);
        assert_eq!(boolean_evaluation::eval_formula("10="), false);
        assert_eq!(boolean_evaluation::eval_formula("101|&"), true);
    }

    #[test]
    fn sat_test() {
        assert_eq!(sat::sat("AB|"), true);
        assert_eq!(sat::sat("AB&"), true);
        assert_eq!(sat::sat("AA!&"), false);
        assert_eq!(sat::sat("AA^!"), false);
    }
}

