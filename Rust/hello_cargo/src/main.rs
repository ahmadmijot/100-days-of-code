/*
Ownership
Understanding stack and heap concept
Stack: LIFO
Heap: allocating memory
Heap slower than stack

*/

/*m
fn main() {
   // scope
   /*
    let mut s = String::from("hello"); //String: allocate unknown amt of memomy in heap
    s.push_str(", world!"); //push_str() appends literal to String
    println!("{}",s);
    */

// move: != shallow copy
/*
    let x = 5;
    let y = x;
    println!("{}, {}", x,y);

    let s1 = String::from("hello");
    println!("{}", s1);
    let s2 = s1; // s1 move into s2, s1 no longer valid
    println!("{}", s2);
*/

// clone: heap data gets deep copy
   /* let s1 = String::from("clone is deep copy");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    */

// copy
/*
any group of simple scalar values can be Copy,
and nothing that requires allocation or is some
form of resource is Copy
*/

// Ownership and fn
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into fn and so no longer valid here
    let x = 5; // x comes into scope

    makes_copy(x); // x wouls move into the fn, but i32 is Copy
                    // so it's ok to still use x afterward

}

//Ownership cont

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope and 'drop' is called. 

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
}  // here some_integer goes out of scope, nothing special happens

m*/

/* 
// Return values and scope
// Return values and transfer ownership

fn main() {
    let s1 = gives_ownership; // gives_ownership moves its return value into s1
    let s2 = String::from("return values and scope"); //s2 comes into scope
    let s3 = takes_and_gives_back(s2);  //s2 is moved into takes_and_gives_back
                                        //which also moves its return value into
                                        //s3
}   // here s3 is out of scope and dropped. s2 out of scope but was moved
    // so nothing happened. s1 out of scope and dropped.

fn gives_ownership() -> String {    //gives_ownership move its return value into
                                    // the fn that calles it
    let some_string = String::from("hello"); // some_string into scope
    some_string                              // some_string is returned ans moved
                                             // out to calling fn
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string //a_string returned and moves out to calling fn
}

*/

// References and Borrowing
// & = references, refer value w/o taking ownership
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length if {} is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { //s is reference to a String
    s.len()
} // Here, s goes out of scope but because it does not have ownership of 
    // what it refers to, nothing happens.