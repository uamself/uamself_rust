fn main() {
    let mut vec = vec![1, 3, 5, 7, 9];
    loop {
        match vec.pop() {
            None => {
                break;
            }
            Some(value) => {
                print!("{} ", value);
            }
        }
    }

    // ===================================
    let mut vec = vec![1, 3, 5, 7, 9];

    while let Some(value) = vec.pop() {
        print!("{} ", value);
    }
}