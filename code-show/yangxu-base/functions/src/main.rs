fn main() {
    print_labeled_measurement(5, 'h');

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

// 传入参数
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// 返回值类型
fn plus_one(x: i32) -> i32 {
    x + 1  // 表达式
}
