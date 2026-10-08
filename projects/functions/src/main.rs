fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    let y = loop {
        break 10;
    };
    println!("The result is {result}, and y is {y}");
}
