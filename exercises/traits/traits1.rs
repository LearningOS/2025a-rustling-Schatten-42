// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.

trait AppendBar {
    fn append_bar(self) -> Self;
}

// 单个 impl 为单个结构体实现一组方法
// impl <traits> for <类型> 实现本类型的一组接口（其实和非traits方法差不多，区别在后头（traits3.rs））
impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
    fn append_bar(mut self) -> Self {
      self.push_str("Bar");
      self
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
