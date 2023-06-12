pub mod introduction;
pub mod rewrite_rule;
pub mod set_theory;
pub mod spaces_filling_curves;

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
