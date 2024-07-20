mod algorithms;

fn main() {
    let numbers: [i32; 4] = [50, 100, 200, 1000];

    println!("index of element: {}", algorithms::binary_search(&numbers, 1000))

}
