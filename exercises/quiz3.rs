// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

use std::fmt;
pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

//impl<T: fmt::Display> 就是“trait bound 语法”。
// 它告诉编译器：* 只有那些实现了 Display 的具体类型才被允许来替换 T *
impl<T: fmt::Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
//使用时“告诉编译器我到底想替换成谁”
// Rust 的泛型是单态化（monomorphization）：
// - 编译期把泛型代码按你实际用的类型各复制一份专门版本，因此零运行时开销。
// 既然要复制，就得让编译器知道“复制成啥”。
// 有两种等价写法：
// 显式写：       let card1 = ReportCard::<f32> { grade: 2.1, ... };
// 让编译器推导： let card2 = ReportCard { grade: "A+", ... }; // 字符串字面量 → &str

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard::<f32> { // 也可以不写 `::<>`
          grade: 2.1,
          student_name: "Tom Wriggle".to_string(),
          student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
