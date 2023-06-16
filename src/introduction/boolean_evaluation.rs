struct AstNode {
    item: char,
    left_leaf: Option<Box<AstNode>>,
    right_leaf: Option<Box<AstNode>>,
}

impl AstNode {
    fn new(item: char) -> AstNode {
        return AstNode {
            item: (item),
            left_leaf: (None),
            right_leaf: (None),
        };
    }

    fn parse_formula(&mut self, formula: &mut Vec<char>) {
        let operand: Vec<char> = vec!['!', '&', '|', '^', '>', '='];
        if formula.len() == 0 {
            return;
        }
        self.item = formula.last().copied().unwrap();
        let c: char = formula.pop().unwrap();
        if operand.iter().any(|&i| i == c) {
            self.left_leaf = Some(Box::new(AstNode::new('0')));
            if c != '!' {
                self.right_leaf = Some(Box::new(AstNode::new('0')));
                self.right_leaf.as_mut().unwrap().parse_formula(formula);
            }
            self.left_leaf.as_mut().unwrap().parse_formula(formula);
        }
    }

    fn compute(&mut self) -> bool {
        if self.item == '1' || self.item == '0' {
            return self.item == '1';
        }

        if self.item == '!' {
            return !self.left_leaf.as_mut().unwrap().compute();
        }
        let left_expr = self.left_leaf.as_mut().unwrap().compute();
        let right_expr = self.right_leaf.as_mut().unwrap().compute();
        match self.item {
            '&' => return left_expr & right_expr,
            '|' => return left_expr | right_expr,
            '^' => return left_expr ^ right_expr,
            '>' => return !(left_expr && !right_expr),
            '=' => return left_expr == right_expr,
            _ => return false,
        }
    }
}

pub fn eval_formula(formula: &str) -> bool {
    let mut formula_stack: Vec<char> = formula.chars().collect();
    let mut root = AstNode::new('0');
    root.parse_formula(&mut formula_stack);
    return root.compute();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_formula_with_and() {
        assert_eq!(eval_formula("10&"), false);
        assert_eq!(eval_formula("00&"), false);
        assert_eq!(eval_formula("00&!"), true);
        assert_eq!(eval_formula("11&"), true);
    }

    #[test]
    fn eval_formula_with_or() {
        assert_eq!(eval_formula("10|"), true);
        assert_eq!(eval_formula("10|!"), false);
        assert_eq!(eval_formula("11|"), true);
        assert_eq!(eval_formula("00|"), false);
    }

    #[test]
    fn eval_formula_with_material() {
        assert_eq!(eval_formula("11>"), true);
        assert_eq!(eval_formula("10>"), false);
        assert_eq!(eval_formula("10>!"), true);
        assert_eq!(eval_formula("00>"), true);
    }

    #[test]
    fn eval_formula_with_equal() {
        assert_eq!(eval_formula("10="), false);
        assert_eq!(eval_formula("11="), true);
        assert_eq!(eval_formula("00="), true);
        assert_eq!(eval_formula("00=!"), false);
    }

    #[test]
    fn eval_formula_complex() {
        assert_eq!(eval_formula("101|&"), true);
        assert_eq!(eval_formula("101|&!"), false);
        assert_eq!(eval_formula("1011||="), true);
        assert_eq!(eval_formula("1011||=!"), false);
    }

    #[test]
    fn eval_formula_xor() {
        assert_eq!(eval_formula("00^"), false);
    }

    #[test]
    fn eval_formula_unique() {
        assert_eq!(eval_formula("0"), false);
        assert_eq!(eval_formula("1"), true);
        assert_eq!(eval_formula("1!"), false);
        assert_eq!(eval_formula("0!"), true);
    }
}
