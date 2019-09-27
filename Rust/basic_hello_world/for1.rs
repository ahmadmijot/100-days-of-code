// Looping ans lfing
fn main () {
    for i in 0..5 {
        println!("Hello {}", i); // range is not inclusive, i=0to 4
    }
println!(""); //don't forget semicolon!!!
    // conditional, method 1
    for i in 0..5 {
        if i % 2 == 0 {
            println!("even {}", i);
        } else {
            println!("odd {}", i);
        }
    }
println!("");

    //conditional, rust-y method
    for i in 00..5 {
        let even_odd = if i % 2 ==0 {"even"} else {"odd"};
        println!("{} {}", even_odd, i)
    }
}