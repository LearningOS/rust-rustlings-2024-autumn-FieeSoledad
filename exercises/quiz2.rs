// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function as described above.
    //在这里我需要使用到input中的String，使用时要避免中间对象立即失效，导致的切片指向无效的内存
    //这里之所以报错是因为我返回的是&str一个引用诶，肯定是有问题的
    pub fn transformer(input: Vec<(String,Command)>) -> [String;4] { 
        let mut ans:[String;4] = Default::default();
        
        //在遍历的同时获得索引
        //从这里理解集合中元素所有权的转移
        for (idx,value) in input.iter().enumerate(){
            match value.1{
                //这个返回的就是&str，这里value遍历后失效，但是trim()返回的内容已经赋值到了数组中
                //这个trim()的底层原理是什么呢，这里trim()
                //这个trim()返回的切片还是原来的String的内存啊
                //trim()返回的确实是一段引用，但是to_string()还是会创建一个新的String对象并进行数据拷贝
                Command::Trim => ans[idx] = value.0.trim().to_string(),
                //这里切片指向的内存很快被释放，切片指向的内容是无效的
                Command::Uppercase => ans[idx] = value.0.to_uppercase(),
                //增长多少倍
                //String.as_str()返回的还是对象指向的那块内存，所以cur很快释放
                Command::Append(size) => {
                    let mut i = 0;
                    //value本是不可变引用，没有所有权
                    let mut cur = value.0.clone();
                    while i<size{
                        cur.push_str("bar");
                        i+=1;
                    }
                    ans[idx] = cur;
                },
            }
        }
        ans
    }

}

fn main() {
    // You can optionally experiment here.
}


//小测2
#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // use ???;
    use super::Command;
    //把transformer这个条目导入
    use super::my_module::transformer;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        for item in &output{
            println!("{}",item)
        }

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
