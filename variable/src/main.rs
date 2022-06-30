fn main() {
    let x = 5;
    let x = x + 1;
    let _space = " ";
    _space = 3;
    {
        let x = x + 3;
        println!("The value of x is: {}", x);
    }
    println!("The value of x is: {}", x);
}
