/**
 * 1、rust中，是「给变量绑定」，而不是给变量赋值，这是为了突出rust中所有权的概念
 * 2、rust中，变量默认不可修改，let表示不可修改，let mut表示可修改
 * 3，下划线开头，表示
 */
struct Struct {
    e: i32
}

// 常量(const)可以在任意位置声明，而不可变变量（let）则不行
// let a = 5;
const A: usize  = 5;

fn main() {
    let (a, mut b): (bool,bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);

    let (a, b, c, d, e);  

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    (c, .., d, _) = (1, 2, 3, 4, 5);
    Struct { e } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    shadowing();
}

fn shadowing() {
    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // shadowing一大作用就是复用变量名
    // 字符串类型
    let spaces = "   ";
    // usize数值类型
    let spaces: usize = spaces.len();

    // let mut的二次赋值，不能修改数据类型
    // let mut spaces = "   ";
    // spaces = spaces.len();

    let mut spaces = "   ";
    let mut spaces = "abc";
    let spaces: usize = spaces.len();

    println!("{}", spaces);

}