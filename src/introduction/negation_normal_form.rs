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
        // DO STUFF
        if self.item == '!' && self.left_leaf.as_ref().unwrap().is_in("&|"){
            // Save node right
            let right_cpy = self.left_leaf.as_ref().unwrap().right_leaf;
            // change root (!) to operator (|& etc...)
            if self.left_leaf.as_ref().unwrap().item == '|' {
                self.item = '&';
            } else {
                self.item = '|';
            }
            // change old operator to !
            self.left_leaf.as_mut().unwrap().item = '!';
            // delete old operator link to A
            self.left_leaf.as_mut().unwrap().right_leaf = None;
            // create a left child to root (the new operator) that point on ! that point himself to A
            self.right_leaf = Some(Box::new(AstNode::new('!')));
            self.right_leaf.as_mut().unwrap().left_leaf = right_cpy;
            // PYTHON ADVANTAGE DON'T BREAK MY BALLS EVERY SINGLE LINE WROTED
        }
        print!("{}", self.item);
    }
}

pub fn negation_normal_form(formula: &str) -> String {
   let mut formula_stack: Vec<char> = formula.chars().collect();
   let mut root = AstNode::new('0');
   root.parse_formula(&mut formula_stack);
   root.negation_normal_form();
   println!("");
   String::from("coucou")
}


fn main() {
    println!("{}", negation_normal_form("AB|!"))
}
