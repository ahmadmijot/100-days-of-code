/*
Rust flows
Rust loop: loop, while, for
*/

fn main() {
    flow_func();
    loop_func();
}

fn flow_func() {
    let condition = true;
    let number = if condition { "six" } else { "five" };

    println!("The value of number is: {}", number);
}

fn loop_func() {
    // loop until explicitely tell to stop

    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("{}", counter);

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
