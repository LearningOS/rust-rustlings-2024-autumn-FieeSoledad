// macros1.rs
//
//说明这是一个声明式宏
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // TODO: Fix the macro call.
    my_macro!()
}
