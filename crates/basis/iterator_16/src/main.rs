fn main() {
    let vec_adapter = vec![1, 3, 5, 7, 9];

    let mut take_result = vec_adapter.iter().take(2);

    for x in take_result {
        println!("{}", x); // 1 3
    }
}