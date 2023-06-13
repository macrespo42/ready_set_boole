use std::collections::HashSet;

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

fn eval_formula(formula: &str) -> bool {
    let mut formula_stack: Vec<char> = formula.chars().collect();
    let mut root = AstNode::new('0');
    root.parse_formula(&mut formula_stack);
    return root.compute();
}

fn print_truth_table_header(formula: &str) {
    print!("|");
    let header: Vec<char> = formula.chars().filter(|&ch| ch.is_alphabetic()).collect();
    for letter in header.iter() {
        print!(" {} |", letter);
    }
    println!(" = |");
    print!("|");
    for _n in 0..header.len() {
        print!("---|");
    }
    print!("---|");
    println!("");
}

fn remove_letter_duplicates(input: &str) -> String {
    let mut unique_letters = HashSet::new();
    let mut result = String::new();

    for ch in input.chars() {
        if ch.is_alphabetic() {
            if unique_letters.insert(ch) {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }

    result
}

fn generate_formulas(expression: &str) -> Vec<String> {
    let mut formulas: Vec<String> = Vec::new();

    let mut variables: Vec<char> = expression
        .chars()
        .filter(|&ch| ch.is_alphabetic())
        .collect();

    variables.sort();
    variables.dedup();

    let num_combinations = 2u32.pow(variables.len() as u32);

    for i in 0..num_combinations {
        let mut formula = String::new();

        for ch in expression.chars() {
            match ch {
                'A'..='Z' => {
                    let var_idx = variables.iter().position(|&v| v == ch).unwrap();
                    let value = (i >> (variables.len() - var_idx - 1)) & 1;
                    formula.push_str(&value.to_string());
                }
                _ => formula.push(ch),
            }
        }

        formulas.push(formula);
    }

    formulas
}

pub fn print_truth_table(formula: &str) {
    let deduplicate_formula = remove_letter_duplicates(formula);
    print_truth_table_header(&deduplicate_formula);
    let expressions = generate_formulas(&deduplicate_formula);
    for formula in expressions {
        for c in formula.chars() {
            if c.is_numeric() {
                print!("| {} ", c);
            }
        }
        println!("| {} |", if eval_formula(&formula) { '1' } else { '0' });
    }
}

fn main() {
    let expression = "AA&";
    let expression = remove_letter_duplicates(expression);
    let formulas = generate_formulas(&expression);

    for formula in formulas {
        println!("{}", formula);
    }
    // print_truth_table("AB&");
    // print_truth_table("QT&");
    // print_truth_table("AB>");
    // print_truth_table("SS=");
    // print_truth_table("EX>R=Y^");
    print_truth_table("AA&");
}
