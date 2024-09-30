# rust

- Reference - dev

  [rust book](https://doc.rust-lang.org/book/), [rust book zh](https://kaisery.github.io/trpl-zh-cn/), 
  
  [cargo book](https://doc.rust-lang.org/cargo/getting-started/index.html), [std book](https://doc.rust-lang.org/std/prelude/index.html)
  
- Reference - course

  [WordScenesTV](https://www.bilibili.com/video/BV1xz421a7ea/), [Muli_Official](https://www.bilibili.com/video/BV16e411n7h4/), [YuanZi](https://www.bilibili.com/video/BV1fF411n7nE/), [Forever-me](https://www.bilibili.com/video/BV1eK4y1b7DE/), 
  
- Reference - blog

  [QingWa](https://blog.frognew.com/2020/07/rust-basis-learning-notes-01.html) 





## 背景介绍

- rust

  令人兴奋的新编程语言 (对标c和c++)

  能让每个人都编写出可靠且高效的软件 (超高性能 编译时消灭)

- rust适用

  高性能、运行时速度极快

  内存安全

  更好地利用多处理器

- rust场景

  高性能web service、webAssembly

  命令行工具、网络编程

  嵌入式编程、系统编程

- 编程语言

  c, c++：*性能极好*；但类型系统和内存不安全

  java, c#：性能稍逊；*保证内存安全、且有很多优秀特性* (Garbage Collection)

  rust：性能极高；无需GC、代码安全性高；*易于维护调试*





### 环境准备

- 环境

  编码环境：vscode; idea

  编译环境：rustc

  包管理器：cargo

  ```bash
  # #################################################
  # install
  # #################################################
  
  # win install
  # rustup-init.exe
  
  # wsl install 
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  
  # linux install
  curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
  
  
  # #################################################
  # check
  # #################################################
  
  rustc --version  # rustc 1.76.0 (07dca489a 2024-02-04)
  cargo --version  # cargo 1.76.0 (c84b36747 2024-01-18)
  
  # check path
  echo %PATH%  # win cmd
  echo $env:Path  # win ps
  echo $PATH  # linux 
  
  
  # #################################################
  # update uninstall 
  # #################################################
  rustup update
  rustup self uninstall
  
  
  # #################################################
  # doc
  # #################################################
  rustup doc  # file:///C:/Users/huangyingzhu/.rustup/...
  cargo doc --open
  
  ```
  
  



#### rustc 

- rustc

  ```bash
  mkdir hello_rust 
  touch hello_rust/main.rs
  
  rustc main.rs
  ./main
  
  ```

  



#### cargo ✔

- cargo

  ```bash
  cargo new hello_cargo
  
  # 编译 + 运行
  cargo build 
  .\target\debug\hello_cargo.exe
  
  # 为发布优化编译
  cargo build --release
  # 确保能编译 (不产生)
  cargo check
  
  # 运行 = 编译 + 运行
  cargo run
  
  ```
  
  



### 快速入门

- 知识点

  `let`、`match`、方法（method）、关联函数（associated function）、外部 crate 等知识

- 任务

  猜数字游戏





#### 新建项目

- 新建

  ```bash
  cargo new guessing_game
  code guessing_game
  
  cargo build
  cargo run
  
  ```

  



#### 一次猜测

- main

  ```rust
  use std::io;
  
  fn main() {
      println!("Guess the number!");
  
      // guess number 
      println!("Please input your guess.");
      let mut guess = String::new();
      io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read line");
      println!("You guessed: {}", guess);
  }
  
  ```

  



#### 生成一个随机数 (rand)

- mian

  ```rust
  use rand::Rng;
  use std::io;
  
  fn main() {
      println!("Guess the number!");
  
      // secret number
      let secret_number = rand::thread_rng().gen_range(1..=100);
      println!("The secret number is: {secret_number}");
  
      // guess number 
      println!("Please input your guess.");
      let mut guess = String::new();
      io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read line");
      println!("You guessed: {}", guess);
  }
  
  ```

  



#### 比较两数 (match)

- mian

  ```rust
  use rand::Rng;
  use std::{cmp::Ordering, io};
  
  fn main() {
      println!("Guess the number!");
  
      // secret number (u32)
      let secret_number = rand::thread_rng().gen_range(1..=100);
      println!("The secret number is: {secret_number}");
  
      // guess number (str)
      println!("Please input your guess.");
      let mut guess = String::new();
      io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read line");
      // guess number (u32)
      let guess: u32 = guess.trim().parse().expect("Please type a number!");
      println!("You guessed: {}", guess);
  
      // compare guess and secret number
      match guess.cmp(&secret_number) {
          Ordering::Less => println!("Too small!"),
          Ordering::Greater => println!("Too big!"),
          Ordering::Equal => println!("You win!"),
      }
  }
  
  ```

  



#### 多次猜测 (loop)

- main

  ```rust
  use rand::Rng;
  use std::{cmp::Ordering, io};
  
  fn main() {
      println!("Guess the number!");
  
      // secret number (u32)
      let secret_number = rand::thread_rng().gen_range(1..=100);
      // println!("The secret number is: {secret_number}");
  
      loop {
          // guess number (str)
          println!("Please input your guess.");
          let mut guess = String::new();
          io::stdin()
              .read_line(&mut guess)
              .expect("Failed to read line");
          // guess number (u32) [continue]
          let guess: u32 = match guess.trim().parse() {
              Ok(num) => num,
              Err(_) => continue,
          };
          println!("You guessed: {}", guess);
  
          // compare guess and secret number [break]
          match guess.cmp(&secret_number) {
              Ordering::Less => println!("Too small!"),
              Ordering::Greater => println!("Too big!"),
              Ordering::Equal => {
                  println!("You win!");
                  break;
              }
          }
      }
  }
  
  ```

  



### 编程概念

- 编程概念

  关键字

  变量和可变性

  





#### 变量和可变性

- 变量和可变性

  **不可变变量**：`let` 变量默认是不可改变的 immutable (当变量不可变时，一旦值被绑定一个名称上，就不能改变这个值)

  **可变变量**：在变量名前添加 `mut` 来使其可变

- 常量和隐藏

  **常量 constants**：`const` 绑定到一个名称的不允许改变的值 (全局常量)

  可以定义一个与之前变量同名的*新变量* `let`，第一个变量被第二个 **隐藏 Shadowing** 了





---

- 新建

  ```bash
  cargo new variables
  code variables
  
  ```

- demo

  变量默认是不可改变的

  ```rust
  fn main() {
      let x = 5;
      println!("The value of x is: {x}");
      // x = 6;  // cannot assign twice to immutable variable
      println!("The value of x is: {x}");
  }
  
  ```

  在变量名前添加 `mut` 来使其可变

  ```rust
  fn main() {
      let mut x = 5;
      println!("The value of x is: {x}");
      x = 6;
      println!("The value of x is: {x}");
  }
  
  ```

  全局常量

  ```rust
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  
  ```

  Shadowing

  ```rust
  fn main() {
      let x = 5;
  
      let x = x + 1;  // 6
  
      {
          let x = x * 2;  // 12
          println!("The value of x in the inner scope is: {x}");  
      }
  
      println!("The value of x is: {x}");  // 6
  }
  
  ```

  



---

- 区别：隐藏和可变变量

  ```rust
  
  fn main() {
      // shadowing (let)
      let spaces = "   ";  // &str
      let spaces = spaces.len();  // usize
  
      // mut
      let mut spaces = "   ";
      // spaces = spaces.len();  // expected `&str`, found `usize`
  }
  
  ```

  



#### 数据类型 (已知大小 stack)

-  数据类型 data type

  标量 scalar：整型、浮点型、布尔类型、字符类

  复合 compound：元组tuple、数组array

  



---

- 整型

  `i8` (有符号 8bit)、`i16`、`i32` (*默认*)、`i64`、`i128`、`isize` (某些集合的索引)

  `u8` (无符号/无负数)、`u16`、`u32`、`u64`、`u128`、`usize` (某些集合的索引)

  字面值

  `98_222` (decimal)、`0xff` (hex)、`0o77` (octal)、`0b1111_0000` (binary)

  `b'A'` (单字节字符 仅限u8)

- 浮点型

  `f32` (32位 有符号 单精度)、`f64` (64位 有符号 双精度 *默认*)

- 布尔型

  `bool` (取值 `true` `false`)

- 字符类型

  `char` (单引号声明)

  ```rust
  fn main() {
      let c = 'z';
      let z: char = 'ℤ'; // with explicit type annotation
      let heart_eyed_cat = '😻';
  }
  
  ```

  



---

- tuple `()`

  数组中的每个元素的类型*不必相同*

  *长度固定*：一旦声明，其长度不会增大或缩小

  ```rust
  fn main() {
      // 获取值：模式匹配 pattern matching (解构destructure元组值)
      let tup: (i32, f64, u8) = (500, 6.4, 1);
      let (x, y, z) = tup;
      println!("The value of y is: {}", y);
  
      // 获取值：索引
      let x: (i32, f64, u8) = (500, 6.4, 1);
      let five_hundred = x.0;
      let six_point_four = x.1;
      let one = x.2;
  
      // 不带任何值的元组有个特殊的名称，叫做 单元unit 元组
      // 这种值以及对应的类型都写作 ()，表示空值或空的返回类型
      // 如果表达式不返回任何其他值，则会隐式返回单元值
  }
  
  ```

- array `[]`

  数组中的每个元素的类型*必须相同*

  *长度固定*：一旦声明，其长度不会增大或缩小

  当想要在*栈stack*而不是在堆heap上为数据分配空间

  ```rust
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
  
  ```

- vetor (todo)

  标准库提供的一个 *允许增长和缩小长度*的*类似数组*的集合类型

  



#### 函数

- 函数

  声明函数、传入参数

  函数体 (由一系列的**语句**和一个*可选*的**结尾表达式**构成)

  返回值 (并不对返回值命名，但要在箭头后声明它的类型)

  ```rust
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
  
  ```

- 语句和表达式

  **语句 Statements** 是执行一些操作但*不返回值*的指令 `;`

  **表达式 Expressions** 计算并*产生一个值* (函数调用 宏调用 大括号的块作用域) ✔

  ```rust
  fn main() {  // 函数定义  // 语句
      let y = 6;  // 使用 let 关键字创建变量并绑定一个值  // 语句
  
      let y = {
          let x = 3;
          x + 1  // 表达式
      }; 
  }
  
  ```

  



#### 注释

- 注释

  单行注释

  文档注释





#### 控制流

- 条件

  if、else if、let(三元运算符)

  ```rust
  fn main() {
      // if; else if
      let number = 6;
      if number % 4 == 0 {  // 条件必须是bool  // 不会尝试自动地将非布尔值转换为布尔值
          println!("number is divisible by 4");
      } else if number % 3 == 0 {
          println!("number is divisible by 3");
      } else if number % 2 == 0 {
          println!("number is divisible by 2");
      } else {
          println!("number is not divisible by 4, 3, or 2");
      }
  
      // let if 三元运算符
      let condition = true;
      let number = if condition { 5 } else { 6 };
      println!("The value of number is: {number}");  // 5
  
      // if 需要相同类型
      let condition = true;
      // let number = if condition { 5 } else { "six" };  // expected integer, found `&str`
      println!("The value of number is: {number}");
  }
  
  ```

- 循环

  `loop` 死循环 (break continue label)

  `while` 条件循环

  `for` 遍历集合

  ```rust
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
      // while 遍历集合 (不好)
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
  
  ```

  



## 基础语法

### 所有权 ownership

#### 什么是所有权

- 程序管理运行时内存

  java：具有垃圾回收机制，在程序运行时有规律地寻找不再使用的内存

  c++：程序员必须亲自分配和释放内存

  rust：通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查。如果违反了任何这些规则，程序都不能编译。在运行时，所有权系统的任何功能都*不会减慢程序* (安全且高效)

- 栈Stack与堆Heap

  栈和堆都是代码在运行时可供使用的内存

  |       | 存入                                                         | 访问                         |
  | ----- | ------------------------------------------------------------ | ---------------------------- |
  | stack | 入栈 (所有数据都必须占用*已知且固定的大小* - *速度更快*)     |                              |
  | Heap  | 在堆上分配内存(申请 内存分配器 返回pointer)<br />指向放入堆中数据的指针是已知的并且大小是固定的，可将该指针存储在栈上 | 通过pointer访问 - *速度更慢* |

  当调用一个函数时，传递给函数的值（包括可能指向堆上数据的指针）和函数的局部变量被压入栈中；当函数结束时，这些值被移出栈

  跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上的重复数据的数量，以及清理堆上不再使用的数据确保不会耗尽空间

  所有权的主要目的就是管理堆数据

- 所有权

  **所有权ownership** 是 Rust 用于如何*管理内存*的一组规则

  > 1. Rust 中的每一个值都有一个 **所有者owner**。
  > 2. 值在任一时刻有且只有一个所有者。
  > 3. 当所有者（变量）离开**作用域scope**，这个值将被丢弃。

  



---

- 字符串字面值 `&str`

  即被*硬编码*进程序里的字符串值

  *不可变的*，但并非所有字符串的值都能在编写代码时就知道

- 字符串类型 `String`

  该类型管理被分配到*堆*上的数据，能够存储在编译时*未知大小*的文本

  申请内存(通用)、还回去(自己释放 GC 所有权)

  ```rust
  fn main() {
      // ============================================
      // 字符串类型 `&str`
      // ============================================
      // 变量 s 绑定到了一个字符串字面值
      let s = "hello";  // 不可变
      println!("{}", s);  // hello
  
      // ============================================
      // 字符串类型 `String`
      // ============================================
      // :: 是运算符，允许将特定的 from 函数置于 String 类型的命名空间（namespace）下
      let mut s = String::from("hello");
      s.push_str(", world!"); // push_str() 在字符串后追加字面值
      println!("{}", s); // hello, world!
  }
  
  ```

  



---

- 变量与数据交互的方式 (给变量赋值)

  移动：默认浅拷贝，且作废前一个

  克隆：深拷贝

  ```rust
  fn main() {
      // ============================================
      // 栈数据 - 浅拷贝=深拷贝 (成本问题)
      // ============================================
      // 整数是有已知固定大小的简单值，所以这两个 5 被放入了栈中
      let x = 5; // 将 5 绑定到 x
      let y = x; // 生成一个值 x 的拷贝并绑定到 y  // 栈上数据 [深拷贝=浅拷贝]
      println!("x = {x}, y = {y}");
  
      // ============================================
      // 堆数据 - 默认move/浅拷贝
      // ============================================
      let s1 = String::from("hello"); 
      let s2 = s1;  // 默认move [浅拷贝]
      // println!("s1 = {s1}, s2 = {s2}");  // value borrowed here after move
  
      // ============================================
      // 堆数据 - clone/深拷贝
      // ============================================
      let s3 = s2.clone();  // clone [深拷贝]
      println!("s3 = {s3}, s2 = {s2}");
  
      // Copy trait 的特殊注解
      // 如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然可用
      // Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait
  }
  
  ```

- 所有权与函数 (将值传递给函数)

  ```rust
  fn main() {
      let s = String::from("hello");  // s 进入作用域
      takes_ownership(s);             // s 的值move到函数里 ...
                                      // ... 所以到这里不再有效
  
      let x = 5;                      // x 进入作用域
      makes_copy(x);                  // x 应该move函数里，
                                      // 但 i32 是 Copy 的，
                                      // 所以在后面可继续使用 x
  
  } // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
    // 没有特殊之处
  
  fn takes_ownership(some_string: String) { // some_string 进入作用域
      println!("{}", some_string);
  } // 这里，some_string 移出作用域并调用 `drop` 方法。
    // 占用的内存被释放
  
  fn makes_copy(some_integer: i32) { // some_integer 进入作用域
      println!("{}", some_integer);
  } // 这里，some_integer 移出作用域。没有特殊之处
  
  ```

  返回值和作用域

  ```rust
  fn main() {
      let s1 = gives_ownership();         // gives_ownership 将返回值
                                          // 转移给 s1
  
      let s2 = String::from("hello");     // s2 进入作用域
      let s3 = takes_and_gives_back(s2);  // s2 被移动到takes_and_gives_back 中，
                                          // 它也将返回值move给 s3
  } // 这里，s3 移出作用域并被丢弃。
    // s2 也移出作用域，但已被移走，所以什么也不会发生。
    // s1 离开作用域并被丢弃
  
  fn gives_ownership() -> String {             // gives_ownership 会将
                                               // 返回值move给调用它的函数
  
      let some_string = String::from("yours"); // some_string 进入作用域。
  
      some_string                              // 返回 some_string 
                                               // 并移出给调用的函数
  }
  
  // takes_and_gives_back 将传入字符串并返回该值
  fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
                                                        // 
  
      a_string  // 返回 a_string 并move给调用的函数
  }
  ```

  





#### 引用和借用



#### slice类型







### 结构体 struct

#### 结构体的定义和实例化



#### 结构体示例程序



#### 方法语法

- 方法

  **方法method** 与函数类似：使用 `fn` 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码

  不过方法与函数是不同的：因为方法在*结构体*的上下文中被定义（或*枚举*或 *trait 对象*的上下文），并且它们第一个参数总是 `self`，它代表调用该方法的结构体实例













### 枚举和模式匹配 enum









## 项目管理

### 包 crate 模块管理











### 常见集合











### 错误处理







### 泛型 trait 生命周期







### 自动化测试









### 项目：命令行程序









## 函数式编程

### 迭代器和闭包









### cargo crates.io







### 智能指针









### 无畏并发













### 面向对象







### 模式与模式匹配











### 高级特征













### 项目：多线程 web server











