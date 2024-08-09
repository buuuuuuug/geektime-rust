fn main() {
    let data = vec![10, 42, 9, 8];
    let v = 42;

    // Try using `match` here?
    match find_pos(data, v) {
        Some(pos) => {
            println!("Found {} at {}", v, pos);
        }
        None => {
            println!("{} not found", v);
        }
    }
}

fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    for (pos, item) in data.iter().enumerate() {
        if *item == v {
            return Some(pos);
        }
    }

    None
}
