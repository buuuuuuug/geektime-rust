#[allow(unused_mut)]

fn main() {
    let mut data = vec![1, 2, 3];
    let data1 = vec![&data[0]];
    println!("data[0]: {:p}", &data[0]);

    // Rust 下，不能同时拥有可变引用和只读引用
    // for i in 0..100 {
    //     data.push(i); // 这一步使得 data这个可变引用 “活跃”起来
    // }

    println!("data[0]: {:p}", &data[0]);
    println!("boxed: {:p}", &data1);
}
