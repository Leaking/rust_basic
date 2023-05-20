fn main() {
    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // shadowing一大作用就是服用变量名
    // 字符串类型
    let spaces = "   ";
    // usize数值类型
    let spaces = spaces.len();

    // let mut的二次赋值，不能修改数据类型
    // let mut spaces = "   ";
    // spaces = spaces.len();

}