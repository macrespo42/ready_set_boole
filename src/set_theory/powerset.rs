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
}

fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    let mut result:Vec<Vec<i32>> = Vec::new(); 
    let mut cardinal: u32 = 2;
    cardinal = cardinal.pow(set.len() as u32);
    for subset_elt in 0..cardinal {
        let mut subset: Vec<i32> = Vec::new();
        for elt in 0..set.len() {
            if subset_elt & (1 << elt) > 0 {
               subset.push(elt as i32);
            }
        }
        result.push(subset);
    }
    result
}
