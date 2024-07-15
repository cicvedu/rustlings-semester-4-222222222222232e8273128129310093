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
    // 定义常量 PI，注意 Rust 习惯上使用全大写字母表示常量
    const PI: f32 = std::f32::consts::PI;
    let radius = 5.00f32;

    // 计算面积时使用 PI 常量，并且使用方法调用的方式来计算平方
    let area = PI * radius.powi(2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    );
}

