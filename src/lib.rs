pub mod introduction;
pub mod rewrite_rule;
pub mod set_theory;

#[allow(unused_imports)]
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
