use std::collections::HashSet;

struct AstNode {
    item: char,
    left_leaf: Option<Box<AstNode>>,
    right_leaf: Option<Box<AstNode>>,
}

impl AstNode {
    fn new(item: char) -> AstNode {
        return AstNode { item: (item), left_leaf: (None), right_leaf: (None) };
    }

    fn parse_formula(&mut self, formula: &mut Vec<char>) {
        let operand: Vec<char> = vec!['!', '&', '|', '^', '>', '='];
        self.item = formula.last().copied().unwrap();
        let c: char = formula.pop().unwrap();
        if operand.iter().any( |&i| i == c) {
            self.left_leaf = Some(Box::new(AstNode::new('0')));
            if c != '!' {
                self.right_leaf = Some(Box::new(AstNode::new('0')));
                self.right_leaf.as_mut().unwrap().parse_formula(formula);
            }
            self.left_leaf.as_mut().unwrap().parse_formula(formula);
        }
    }

    fn compute(&mut self) -> Vec<i32> {
        let result: Vec<i32> = Vec::new();
        // do thing 
        result 
    }
}

fn is_valid_formula(formula: &str, sets_size: usize) -> bool {
    formula.chars().collect::<HashSet<_>>().iter().filter(|c| c.is_alphabetic()).count() == sets_size
}

pub fn eval_set(formula: &str, sets: &Vec<Vec<i32>>) -> Vec<i32> {
    if !is_valid_formula(formula, sets.len()) {
        panic!("The formula and sets provided are not compatible");
    } else {
        let mut formula_stack: Vec<char> = formula.chars().collect(); 
        let mut root = AstNode::new('0');
        root.parse_formula(&mut formula_stack);
        root.compute()
    }
}

fn main() {
    let sets = vec![
        vec![0, 1, 2],
        vec![0, 3, 4],
    ];

    println!("{:?}", eval_set("AB&", &sets));
}
