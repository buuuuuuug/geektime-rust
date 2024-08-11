fn main() {
    let mut x = 42;

    let r1 = &mut x;
    // reborrow 可以通过
    let r2 = &mut *r1;
    // &x 不可以
    // let r2 = &x;

    // println!("r1: {:p}, r2: {:p}", r1, &r2);

    // *r1 += 1;
    *r2 += 1;
    x += 1;

    println!("r2: {r2}")
}
