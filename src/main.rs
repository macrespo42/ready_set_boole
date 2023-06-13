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
}
