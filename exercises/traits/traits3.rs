// traits3.rs
//
// Your task is to implement the Licensed trait for both structures and have
// them return the same information without writing the same function twice.
//
// Consider what you can add to the Licensed trait.
//
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a
// hint.


pub trait Licensed {
    fn licensing_info(&self) -> String {
      "Some information".to_string()
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Don't edit this line
impl Licensed for OtherSoftware {} // Don't edit this line
// impl <traits> {直接实现}，加上上面两行空 impl，可以为一堆结构体实现统一的接口
// ** 重要：不能为 外部 实现 外部trai ts ** ==> 相干性(coherence)/孤儿规则(orphan rule)
// 确保了别人编写的代码不会破坏我的代码

// - 你可以为 内部类型 SomeSoftware 实现外部接口  fmt::Display
// - 也可以为 外部类型 Vec<T> 实现内部接口  append_bar
// - 更可以为 内部类型实现内部接口...


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
