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
        let c: char = formula.pop().unwrap()
        if operand.iter().any( |&i| i == c) {
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

fn eval_formula(formula: &str) -> bool {
    let mut formula_stack: Vec<char> = formula.chars().collect(); 
    let mut root = AstNode::new('0');
    root.parse_formula(&mut formula_stack);
    return root.compute();
}

fn print_truth_table_header(formula: &str) {
    print!("|");
    let mut header: Vec<char> = formula.chars().collect();
    header.sort_unstable();
    header.dedup();
    for letter in header.iter() {
        if letter.is_alphabetic() {
            print!(" {} |", letter);
        }
    }
    println!(" = |");
    print!("|");
    for _n in 1..header.len() {
        print!("---|");
    }
    println!("");
}

fn generate_combinations(formula: &str) -> Vec<String> {
    let mut result_set: HashSet<String> = HashSet::new();
    generate_combinations_helper(formula, &mut result_set);
    result_set.into_iter().collect()
}

fn generate_combinations_helper(formula: &str, result_set: &mut HashSet<String>) {
    if formula.is_empty() {
        result_set.insert("".to_string());
        return;
    }
    let rest = &formula[1..];
    let sub_combinations = generate_combinations(rest);
    let current_char = formula.chars().next().unwrap();
    if current_char.is_alphabetic() {
        for sub_combination in sub_combinations {
            result_set.insert(format!("{}{}", '0', sub_combination));
            result_set.insert(format!("{}{}", '1', sub_combination));
        }
    } else {
        for sub_combination in sub_combinations {
            let new_sub_combination = format!("{}{}", current_char, sub_combination);
            result_set.insert(new_sub_combination);
        }
    }
}

pub fn print_truth_table(formula: &str) {
    let mut formula_stack: Vec<char> = formula.chars().collect(); 
    let mut root = AstNode::new('0');
    root.parse_formula(&mut formula_stack);
    print_truth_table_header(formula);
    let combinations = generate_combinations(formula);
    for combination in combinations {
        for c in combination.chars() {
            if c.is_numeric() {
                print!("| {} ", c);
            }
        }
        println!("| {} |", if eval_formula(&combination) {'1'} else {'0'});
    }
}

fn main() {
    print_truth_table("ABA&|");
}
