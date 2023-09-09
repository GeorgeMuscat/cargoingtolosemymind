fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let mut letters: &str = "abcde";
    println!("{}", letters);
    letters = "abcdef";
    println!("{}", letters);

    let flt: f64 = 0.2;
    println!("{}", flt);
    let flt: f64 = flt + 0.1;
    println!("{}", flt);
}
