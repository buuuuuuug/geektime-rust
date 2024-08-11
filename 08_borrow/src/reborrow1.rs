fn main() {
    let mut data = vec![1, 2, 3, 4];
    let b = & data;
    println!("addr of the ref b: {:p}", &b);
    println!("sum of data1: {}", sum(b));
    // ok
    println!("{:?}", b);
}

fn sum(v: & Vec<i32>) -> &i32 {
    println!("addr of the ref v: {:p}", &v);
    let sum = v.iter().sum();
    &sum // 可以返回 sum 但是不能返回 sum 的引用： 因为i32类型实现了CopyTrait，返回的数据是有所有权的，但是返回引用会在函数结束时销毁
}
