// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
// 为字符串向量（Vec<String>）实现 AppendBar trait
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        // 将接收的不可变向量转为可变向量（因要修改内容）
        let mut mutable_vec = self;
        // 向向量末尾追加 "Bar"（需转为 String 类型以匹配向量元素类型）
        mutable_vec.push(String::from("Bar"));
        // 返回修改后的向量，满足 trait 方法“接收 self 并返回 Self”的签名
        mutable_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
