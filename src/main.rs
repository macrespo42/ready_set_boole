use std::{io, vec};
mod introduction;
mod spaces_filling_curves;

use ready_set_boole::rewrite_rule::conjuctive_normal_form::conjunctive_normal_form;
use ready_set_boole::rewrite_rule::negation_normal_form::negation_normal_form;
use ready_set_boole::rewrite_rule::sat;
use ready_set_boole::set_theory::powerset::powerset;
use ready_set_boole::set_theory::set_evalutation::eval_set;

use crate::introduction::adder::adder;
use crate::introduction::boolean_evaluation::eval_formula;
use crate::introduction::gray_code::gray_code;
use crate::introduction::multiplier::multiplier;
use crate::introduction::truth_table::print_truth_table;
use crate::spaces_filling_curves::curve::map;
use crate::spaces_filling_curves::inverse_function::reverse_map;

fn main() {
    println!("ready... set... boole! 🏁 Choose the exercice you want to check:");
    println!("Available:");
    for n in 0..12 {
        if n < 10 {
            println!(" - Exercice 0{n}");
        } else {
            println!(" - Exercice {n}");
        }
    }

    let mut exercice = String::from("");

    io::stdin()
        .read_line(&mut exercice)
        .expect("Failed to read line");

    let exercice: u32 = match exercice.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Bad user input"),
    };

    match exercice {
        0 => {
            println!("\nExercise 00 - Adder");
            println!("-------------------------------------");
            ex00(4, 2);
            ex00(0, 0);
            ex00(4, 0);
            ex00(0, 2);
            ex00(4_000, 2_000);
            ex00(4_0000, 2_0000);
            ex00(4_00000, 2_00000);
        }
        1 => {
            println!("\nExercise 01 - Multiplier");
            println!("-------------------------------------");
            ex01(4, 2);
            ex01(1, 2);
            ex01(3, 3);
            ex01(23, 3);
            ex01(8, 47);
            ex01(0, 2);
            ex01(4, 0);
            ex01(4_00, 2_00);
            ex01(1_000, 1_000);
        }
        2 => {
            println!("\nExercise 02 - Gray code");
            println!("-------------------------------------");
            ex02(0, 0);
            ex02(1, 1);
            ex02(2, 3);
            ex02(3, 2);
            ex02(4, 6);
            ex02(5, 7);
            ex02(6, 5);
            ex02(7, 4);
            ex02(8, 12);
        }
        3 => {
            println!("\nExercise 03 - Boolean evaluation");
            println!("-------------------------------------");
            ex03("10&", false);
            ex03("00&", false);
            ex03("00&!", true);
            ex03("11&", true);
            ex03("10|", true);
            ex03("10|!", false);
            ex03("11|", true);
            ex03("00|", false);
            ex03("11>", true);
            ex03("10>", false);
            ex03("00>", true);
            ex03("10=", false);
            ex03("11=", true);
            ex03("00=", true);
            ex03("00=!", false);
            ex03("101|&", true);
            ex03("101|&!", false);
            ex03("1011||=", true);
            ex03("1011||=!", false);
        }
        4 => {
            println!("\nExercise 04 - Truth table");
            println!("-------------------------------------");
            println!("Truth table of: AB&C|");
            print_truth_table("AB&C|");
            println!("-------------------------------------");
            println!("Truth table of: QT&");
            print_truth_table("QT&");
            println!("-------------------------------------");
            println!("Truth table of: AB>");
            print_truth_table("AB>");
            println!("-------------------------------------");
            println!("Truth table of: SS=");
            print_truth_table("SS=");
            println!("-------------------------------------");
            println!("Truth table of: EX>R=Y^");
            print_truth_table("EX>R=Y^");
            println!("-------------------------------------");
        }
        5 => {
            println!("\nExercise 05 - Negation Normal Form");
            println!("-------------------------------------");
            ex05("AB|!", "A!B!&");
            ex05("AB&!", "A!B!|");
            ex05("AB>", "A!B|");
            ex05("AB=", "A!B|B!A|&");
            ex05("AB|C&!", "A!B!&C!|");
            ex05("AB^", "AB!&A!B&|");
        }
        6 => {
            println!("\nExercise 06 - Conjuctive Normal Form");
            println!("-------------------------------------");
            ex06("AB&!", "A!B!|");
            ex06("AB|!", "A!B!&");
            ex06("AB|C&", "AB|C&");
            ex06("AB|C|D|", "DCAB|||");
            ex06("AB&C&D&", "DCAB&&&");
            ex06("AB&!C!|", "C!A!B!||");
            ex06("AB|!C!&", "C!A!B!&&");
            ex06("ABDE&|&", "ABD|BE|&&");
        }
        7 => {
            println!("\nExercise 07 - SAT");
            println!("-------------------------------------");
            ex07("AB|", true);
            ex07("AB&", true);
            ex07("AA!&", false);
            ex07("AA^", false);
        }
        8 => {
            println!("\nExercise 08 - Powerset");
            println!("-------------------------------------");
            println!("Empty set:");
            println!("{:?}", powerset(&[]));
            println!("-------------------------------------");
            println!("With [42]");
            println!("{:?}", powerset(&[42]));
            println!("-------------------------------------");
            println!("With [1, 2, 3]");
            println!("{:?}", powerset(&[1, 2, 3]));
            println!("-------------------------------------");
        }
        9 => {
            println!("\nExercise 09 - Set evaluation");
            println!("-------------------------------------");
            ex09("AB&", &vec![vec![0, 1, 2], vec![0, 3, 4]], vec![0]);
            ex09(
                "AB|",
                &vec![vec![0, 1, 2], vec![3, 4, 5]],
                vec![0, 1, 2, 3, 4, 5],
            );
            ex09("A!", &vec![vec![0, 1, 2]], vec![]);
        }
        10 | 11 => {
            println!("\nExercise 10 / 11 - Curve & Inverse function");
            test_map(0, 0);
            test_map(1, 0);
            test_map(0, 1);
            test_map(100, 100);
            test_map(30000, 1);
            test_map(1, 30000);
            test_map(65534, 65535);
            test_map(65535, 65534);
            test_map(65535, 65535);
        }
        _ => println!("This exercice does not exist or are not implemented yet 🙄"),
    }
}

fn ex00(a: u32, b: u32) {
    println!("{a} + {b}");
    println!("expected: {}", a + b);
    println!("got: {}", adder(a, b));
    println!("-------------------------------------");
}

fn ex01(a: u32, b: u32) {
    println!("{a} + {b}");
    println!("expected: {}", a * b);
    println!("got: {}", multiplier(a, b));
    println!("-------------------------------------");
}

fn ex02(n: u32, expected: u32) {
    println!("Gray code of {n}");
    println!("expected: {expected}");
    println!("got: {}", gray_code(n));
    println!("-------------------------------------");
}

fn ex03(formula: &str, expected: bool) {
    println!("formula: {formula}");
    println!("expected: {expected}");
    println!("got: {}", eval_formula(formula));
    println!("-------------------------------------");
}

fn ex05(formula: &str, expected: &str) {
    println!("nnf of {formula}");
    println!("expected: {expected}");
    println!("got: {}", negation_normal_form(formula));
    println!("-------------------------------------");
}

fn ex06(formula: &str, expected: &str) {
    println!("cff of {formula}");
    println!("expected: {expected}");
    println!("got: {}", conjunctive_normal_form(formula));
    println!("-------------------------------------");
}

fn ex07(formula: &str, expected: bool) {
    println!("SAT of {formula}");
    println!("expected: {expected}");
    println!("got: {}", sat::sat(formula));
    println!("-------------------------------------");
}

fn ex09(formula: &str, sets: &Vec<Vec<i32>>, expected: Vec<i32>) {
    println!("Set evaluation of : {formula} with sets: {:?}", sets);
    println!("expected: {:?}", expected);
    println!("got: {:?}", eval_set(formula, sets));
    println!("-------------------------------------");
}

fn test_map(x: u16, y: u16) {
    println!("Converting {} and {}", x, y);
    let mapped: f64 = map(x, y);
    println!("Converted to {} !", mapped);
    println!("Reversing conversion...");
    let unmapped: (u16, u16) = reverse_map(mapped);
    println!("Number evaluates to x {} and y {}", unmapped.0, unmapped.1);
    println!("-------------------------------------");
}
