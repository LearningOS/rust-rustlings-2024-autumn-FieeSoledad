// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.


// TODO: Fix the compiler error in the `main` function without changing this function.
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green"); // Don't change this line.

    //这里考的是String切片变成&str类型
    if is_a_color_word(&word[0..word.len()]) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}