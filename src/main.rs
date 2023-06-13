use std::io;
mod introduction;
mod spaces_filling_curves;

use crate::introduction::adder::adder;
use crate::introduction::boolean_evaluation::eval_formula;
use crate::introduction::gray_code::gray_code;
use crate::introduction::multiplier::multiplier;
use crate::introduction::truth_table::print_truth_table;

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
