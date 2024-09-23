fn main() {
    // 数量固定 
    let months = [
        "January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"
    ];

    // 声明类型  短创建
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];  // [3, 3, 3, 3, 3]

    // 获取值：索引
    let first = a[0];
    let second = a[1];

    // 索引越界 panic
}
