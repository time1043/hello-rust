fn main() {
    // ============================================
    // loop break continue
    // ============================================
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // break
        }
    };
    println!("The result is {result}");

    // ============================================
    // loop label
    // ============================================
    let mut count = 0;
    'counting_up: loop {  // loop1
        println!("count = {count}");
        let mut remaining = 10;

        loop {  // loop2
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;  // break loop2
            }
            if count == 2 {
                break 'counting_up;  // break loop1
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // ============================================
    // while条件 = loop + if else + break
    // ============================================
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // ============================================
    // while 遍历集合
    // ============================================
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // ============================================
    // for 遍历集合
    // ============================================
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    // Range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}
