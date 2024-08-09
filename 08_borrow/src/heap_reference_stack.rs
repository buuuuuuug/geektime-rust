fn main() {
    let mut data: Vec<&u32> = Vec::new(); // data 分配在堆上
    let mut v = 42; // v 存在于栈
    data.push(&v); // 让data引用v
    println!("data: {:?}", &data);
    v = 44;
    println!("v: {:?}", v);
    // println!("data: {:?}", data); // 编译失败： 借用的生命周期长于原本变量的生命周期
}
