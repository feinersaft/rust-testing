fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    // Here x can be changed due to "mut".
    x = x + 1;
    println!("The value of x is: {}", x);

    // ---

    let x = 5;
    println!("The value of x is: {}", x);
    // Here x can be *not* changed, because it is immutable. We can instead shadow it.
    //x = x + 1; Error
    let x = x + 1;
    println!("The value of x is: {}", x);
}
