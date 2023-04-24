use std::collections::HashSet;

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
            self.left_leaf = Some(Box::new(AstNode::new('0')));
            if c != '!' {
                self.right_leaf = Some(Box::new(AstNode::new('0')));
                self.right_leaf.as_mut().unwrap().parse_formula(formula);
            }
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

    fn compute(&mut self, sets: &Vec<Vec<i32>>, superset: &mut Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        if !self.is_in("&|!") {
           result = sets[self.item as usize - 65].clone();
        }
        else if self.item == '!' {
            for x in sets[self.left_leaf.as_mut().unwrap().item as usize - 65].clone().iter() {
                superset.retain(|elt| *elt != *x);
            }
        }
        else if self.item == '|' {
            let mut set: HashSet<i32> = HashSet::new();
            set.extend(sets[self.left_leaf.as_mut().unwrap().item as usize - 65].clone());
            set.extend(sets[self.right_leaf.as_mut().unwrap().item as usize - 65].clone());
            for x in set.iter() {
                if superset.contains(x) {
                    result.push(*x);
                }
            }
        }
        else if self.item == '&' {
            for x in sets[self.left_leaf.as_mut().unwrap().item as usize - 65].iter() {
               if sets[self.right_leaf.as_mut().unwrap().item as usize - 65].contains(x) && superset.contains(x) {
                   result.push(*x);
               }
            }
        }
        result 
    }
}

fn is_valid_formula(formula: &str, sets_size: usize) -> bool {
    formula.chars().collect::<HashSet<_>>().iter().filter(|c| c.is_alphabetic()).count() == sets_size
}

fn get_superset(sets: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut set: HashSet<i32> = HashSet::new();
    for elements in sets.iter() {
        set.extend(elements.clone());
    }
    let result: Vec<i32> = set.into_iter().collect();
    return result;
}

pub fn eval_set(formula: &str, sets: &Vec<Vec<i32>>) -> Vec<i32> {
    if !is_valid_formula(formula, sets.len()) {
        panic!("The formula and sets provided are not compatible");
    } else {
        let mut formula_stack: Vec<char> = formula.chars().collect(); 
        let mut root = AstNode::new('0');
        root.parse_formula(&mut formula_stack);
        root.negation_normal_form();
        root.compute(sets, &mut get_superset(sets))
    }
}

fn main() {
    let sets = vec![
        vec![0, 1, 2],
        vec![0, 3, 4],
    ];

    println!("{:?}", eval_set("AB&", &sets));

    let sets = vec![
        vec![0, 1, 2],
        vec![3, 4, 5],
    ];

    println!("{:?}", eval_set("AB|", &sets));

    let set = vec![
        vec![0, 1, 2, 4, 5],
    ];

    println!("{:?}", eval_set("A", &set));
    println!("{:?}", eval_set("A!", &set));
}