// traits2.rs
//
trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement the trait `AppendBar` for a vector of strings.
// `append_bar` should push the string "Bar" into the vector.

//如何给一个带有泛型参数的类型实现trait？具体类型和约束？
impl AppendBar for Vec<String>{
    fn append_bar(self)->Self{
        let mut v = self;
        v.push(String::from("Bar"));
        v
    }
} 

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
