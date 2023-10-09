fn main() {
    println!("Aloha~");
    let mut x = 5;// """只有有mut才是可变变量，但是rust里面有常量"""
    println!("the value of x is {}",x);
    x = 6;
    println!("the value of x is {}",x);
    //常量必须标注清楚类型，而且不准使用let只能用const标注

}
