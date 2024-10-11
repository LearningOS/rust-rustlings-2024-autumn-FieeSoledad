// traits1.rs
//
// The trait `AppendBar` has only one function which appends "Bar" to any object
// implementing this trait.

trait AppendBar {
    //这个self代表什么？获取了数据的所有权
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for the type `String`.
    fn append_bar(self)->Self{
        let mut s = self;
        s.push_str("Bar");
        s
    }
}



fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), "FooBar");
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(String::from("").append_bar().append_bar(), "BarBar");
    }
}
