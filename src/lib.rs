pub mod introduction;
pub mod rewrite_rule;
pub mod set_theory;
pub mod spaces_filling_curves;

use crate::introduction::*;
#[allow(unused_imports)]
use crate::rewrite_rule::*;
#[allow(unused_imports)]
use crate::set_theory::*;

#[cfg(test)]
mod introduction_test {
    #[allow(unused_imports)]
    use crate::set_theory::set_evalutation::eval_set;

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
    fn negation_normal_form_test() {
        assert_eq!(negation_normal_form::negation_normal_form("AB|!"), "A!B!&");
        assert_eq!(negation_normal_form::negation_normal_form("AB&!"), "A!B!|");
        assert_eq!(negation_normal_form::negation_normal_form("AB>"), "A!B|");
        assert_eq!(
            negation_normal_form::negation_normal_form("AB="),
            "A!B|B!A|&"
        );
        assert_eq!(
            negation_normal_form::negation_normal_form("AB|C&!"),
            "A!B!&C!|"
        );
        assert_eq!(
            negation_normal_form::negation_normal_form("AB^"),
            "AB!&A!B&|"
        );
    }

    #[test]
    fn conjunctive_normal_form_test() {
        assert_eq!(
            conjuctive_normal_form::conjunctive_normal_form("AB&!"),
            "A!B!|"
        );
        assert_eq!(
            conjuctive_normal_form::conjunctive_normal_form("AB|!"),
            "A!B!&"
        );
        assert_eq!(
            conjuctive_normal_form::conjunctive_normal_form("AB|C&"),
            "AB|C&"
        );
        assert_eq!(
            conjuctive_normal_form::conjunctive_normal_form("AB|C|D|"),
            "DCAB|||"
        );
        assert_eq!(
            conjuctive_normal_form::conjunctive_normal_form("AB&C&D&"),
            "DCAB&&&"
        );
        assert_eq!(
            conjuctive_normal_form::conjunctive_normal_form("AB&!C!|"),
            "C!A!B!||"
        );
        assert_eq!(
            conjuctive_normal_form::conjunctive_normal_form("AB|!C!&"),
            "C!A!B!&&"
        );
        assert_eq!(
            conjuctive_normal_form::conjunctive_normal_form("ABDE&|&"),
            "ABD|BE|&&"
        );
    }

    #[test]
    fn sat_test() {
        assert_eq!(sat::sat("AB|"), true);
        assert_eq!(sat::sat("AB&"), true);
        // assert_eq!(sat::sat("AA!&"), false);
        // assert_eq!(sat::sat("AA^!"), false);
    }

    #[test]
    fn powerset_test() {
        assert_eq!(powerset::powerset(&[]), [[]]);
        assert_eq!(powerset::powerset(&[42]), vec![vec![], vec![42]]);
        assert_eq!(
            powerset::powerset(&[1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
    }

    #[test]
    fn eval_set_test() {
        let sets = vec![vec![0, 1, 2], vec![0, 3, 4]];

        assert_eq!(set_evalutation::eval_set("AB&", &sets), [0]);

        let sets = vec![vec![0, 1, 2], vec![3, 4, 5]];

        let mut result: Vec<i32> = set_evalutation::eval_set("AB|", &sets);
        result.sort();
        assert_eq!(result, vec![0, 1, 2, 3, 4, 5]);

        let sets = vec![vec![0, 1, 2]];

        assert_eq!(set_evalutation::eval_set("A!", &sets), []);
    }
}
