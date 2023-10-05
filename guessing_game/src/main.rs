use std::io;
// io库在标准库std里面 可以用这个库来获取用户的输入并打印出来
//rust会将prelude导入到每个模块的作用域中
//如果类型不在prelude里面，就用use给导入进去
fn main() {
    println!("猜数！");
    //带！是宏
    println!("猜一个整数");
    //let FOO = 1;将1的值赋给了变量FOO
    //let bar = FOO; 将foo的变量值绑定到bar上
    //let FOO = 2;rust里面变量默认不可变，此时会报错，无法两次为不可变变量赋值
    //加上mut这样就可以为可变变量了
    let mut guess = String::new();
    //将guess绑定上标准库字符串的一个实例
    //::关联函数，相当于java静态方法
    io::stdin().read_line(&mut guess).expect("无法读取行");
    //io库stdin 返回Stdin用来处理终端输入
    //read_line 将用户输入放到字符串里
    //&表示引用，不同地方访问同一个内存地址
    //引用默认不可变 加mut使之可变
    //io::Result（readline返回的枚举） 返回OK表示成功，Err表示失败 
    //expect是io::Result定义的方法 err时，中断程序，返回信息，如果ok就会返回附加值给用户
    println!("猜测的数是:{}",guess);
    //{}占位符，对应变量值
}
