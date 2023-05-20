fn main() {
    // 字符必须使用单引号，字符都是4个字节
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));
}