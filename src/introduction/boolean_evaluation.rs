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
            self.right_leaf = Some(Box::new(AstNode::new('0')));
            self.right_leaf.as_mut().unwrap().parse_formula(formula);
            self.left_leaf.as_mut().unwrap().parse_formula(formula);
        }
    }

}

pub fn eval_formula(formula: &str) -> bool {
    let mut formula_stack: Vec<char> = formula.chars().collect(); 
    let mut root = AstNode::new('0');
    root.parse_formula(&mut formula_stack);
    return false;
}

fn main() { 
    println!("{}", eval_formula("101|&"));
}
