// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.


use std::f32;

fn main() {

  // clippy定义的规则: 不让我们用低精度预定义常量
  // ✅ 规则目标
  // 扫描代码中手写的浮点字面量，若其值足够接近标准库已定义的数学常量（如 π、e、ln2 等），则强制要求改用标准库常量，以避免精度损失并提高可读性。
  // 🔍 检查流程(简化):  
  //     收集所有浮点字面量。
  //     与内置常量表（π、e、√2、ln2、ln10 …）做近似比较：
  //         使用 ApproxEq 辅助结构，支持绝对误差与相对误差双阈值 
          // 差值在阈值内 → 触发 lint。

    // let pi = 3.14f32;
    let pi = f32::consts::PI;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
