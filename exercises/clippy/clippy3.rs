// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // 使用 match 或者 if let 来安全地处理 Option
    if my_option.is_none() {
        println!("Expected Some value");
    }

    // 数组声明中补充逗号来正确分隔元素
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 使用 truncate(0) 来清空 Vec
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.truncate(0);
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 使用元组解构来交换两个变量的值
    (value_a, value_b) = (value_b, value_a);
    println!("value a: {}; value b: {}", value_a, value_b);
}


