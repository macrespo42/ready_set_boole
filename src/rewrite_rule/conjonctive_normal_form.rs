#[derive(Clone)]
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
            if c != '!' {
                self.right_leaf = Some(Box::new(AstNode::new('0')));
                self.right_leaf.as_mut().unwrap().parse_formula(formula);
            }
            self.left_leaf = Some(Box::new(AstNode::new('0')));
            self.left_leaf.as_mut().unwrap().parse_formula(formula);
        } 
    }

    fn is_in(&self, haystack: &str) -> bool {
        for c in haystack.chars() {
            if self.item == c {
                return true;
            }
        }
        false
    }

    fn negation_normal_form(&mut self) {
        if self.left_leaf.is_some() {
            self.left_leaf.as_mut().unwrap().negation_normal_form();
        }

        if self.right_leaf.is_some() {
            self.right_leaf.as_mut().unwrap().negation_normal_form();
        }

        if self.item == '!' && self.left_leaf.as_ref().unwrap().is_in("&|"){
            let right_cpy = self.left_leaf.as_mut().unwrap().right_leaf.take();

            if self.left_leaf.as_ref().unwrap().item == '|' {
                self.item = '&';
            } else {
                self.item = '|';
            }

            self.left_leaf.as_mut().unwrap().item = '!';
            self.left_leaf.as_mut().unwrap().right_leaf = None;

            self.right_leaf = Some(Box::new(AstNode::new('!')));
            self.right_leaf.as_mut().unwrap().left_leaf = right_cpy;

            self.negation_normal_form();
        }

        if self.item == '=' {
            self.item = '&';
            let a_cpy = self.left_leaf.take();
            let b_cpy = self.right_leaf.take();

            self.left_leaf = Some(Box::new(AstNode::new('>')));
            self.right_leaf = Some(Box::new(AstNode::new('>')));

            self.left_leaf.as_mut().unwrap().left_leaf = a_cpy.clone();
            self.left_leaf.as_mut().unwrap().right_leaf = b_cpy.clone();

            self.right_leaf.as_mut().unwrap().left_leaf = b_cpy.clone();
            self.right_leaf.as_mut().unwrap().right_leaf = a_cpy.clone();

            self.negation_normal_form();
        }

        if self.item == '^' {
            self.item = '|';
            let a_cpy = self.left_leaf.take();
            let b_cpy = self.right_leaf.take();

            self.left_leaf = Some(Box::new(AstNode::new('&')));
            self.right_leaf = Some(Box::new(AstNode::new('&')));

            self.left_leaf.as_mut().unwrap().right_leaf = Some(Box::new(AstNode::new('!')));
            self.left_leaf.as_mut().unwrap().right_leaf.as_mut().unwrap().left_leaf = b_cpy.clone();
            self.left_leaf.as_mut().unwrap().left_leaf = a_cpy.clone();

            self.right_leaf.as_mut().unwrap().left_leaf = Some(Box::new(AstNode::new('!')));
            self.right_leaf.as_mut().unwrap().left_leaf.as_mut().unwrap().left_leaf = a_cpy.clone();
            self.right_leaf.as_mut().unwrap().right_leaf = b_cpy.clone();
            self.negation_normal_form();
        }

        if self.item == '>' {
            let left_cpy = self.left_leaf.take();
            self.item = '|';
            self.left_leaf = Some(Box::new(AstNode::new('!')));
            self.left_leaf.as_mut().unwrap().left_leaf= left_cpy;
            self.negation_normal_form();
        }
    }

    fn stringify(&mut self) -> String {
        let mut expr = String::from("");

        if self.left_leaf.is_some() {
            expr.push_str(&self.left_leaf.as_mut().unwrap().stringify());
        }

        if self.right_leaf.is_some() {
            expr.push_str(&self.right_leaf.as_mut().unwrap().stringify());
        }

        expr.push(self.item);
        expr
    }

    fn is_conjonctive(&self) -> bool {
        if self.item == '|' {
            if (self.right_leaf.as_ref().unwrap().item == '&' || self.left_leaf.as_ref().unwrap().item == '&') &&
                (self.right_leaf.as_ref().unwrap().is_in("&|") || self.left_leaf.as_ref().unwrap().is_in("&|")) {
                    return false;
                }
        }
        true
    }

    fn conjonctive_normal_form(&mut self) {
        if self.left_leaf.is_some() {
            self.left_leaf.as_mut().unwrap().conjonctive_normal_form();
        } 

        if self.right_leaf.is_some() {
            self.right_leaf.as_mut().unwrap().conjonctive_normal_form();
        }
        if self.is_conjonctive() == false {
            self.item = '&';
            let x_cpy = self.left_leaf.take();
            let a_cpy = self.right_leaf.as_mut().unwrap().left_leaf.take();
            let b_cpy = self.right_leaf.as_mut().unwrap().right_leaf.take();

            self.left_leaf = Some(Box::new(AstNode::new('|')));
            self.right_leaf = Some(Box::new(AstNode::new('|')));

            self.left_leaf.as_mut().unwrap().left_leaf = x_cpy.clone();
            self.left_leaf.as_mut().unwrap().right_leaf = a_cpy;

            self.right_leaf.as_mut().unwrap().left_leaf = x_cpy.clone();
            self.right_leaf.as_mut().unwrap().left_leaf = b_cpy;
        }
    }
}

fn conjonctive_normal_form(formula: &str) -> String {
    let mut formula_stack: Vec<char> = formula.chars().collect();
    let mut root = AstNode::new('0');
    root.parse_formula(&mut formula_stack);
    root.negation_normal_form();
    root.conjonctive_normal_form();
    root.stringify()
}
