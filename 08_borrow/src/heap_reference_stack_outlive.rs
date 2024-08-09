fn main() {
    let mut data: Vec<&u32> = Vec::new();
    push_local_ref(&mut data);
    println!("data: {:?}", data);
}

#[allow(unused_variables)]
fn push_local_ref(data: &mut Vec<&u32>) {
    let v = 42;
    // v 生命周期不够长，如果注释掉会编译不过
    // data.push(&v); // 由于 data生命周期比v长，（v是函数内局部变量，函数结束后释放）
}
