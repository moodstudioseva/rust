fn main() {
    let s = String::from("hello");

    takes_ownship(s);

    let x = 5;
    
    makes_copy(x);
}

fn takes_ownship(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
