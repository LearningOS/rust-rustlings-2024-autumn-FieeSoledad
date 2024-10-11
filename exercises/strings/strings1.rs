// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    //这第一题就是靠&str和String的转换
    "blue".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
