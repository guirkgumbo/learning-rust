fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("The value of x is: {}", x);
    let x = 6;
    println!("The value of x is: {}", x);
    // Const variables cannot be mut and must be annotated
    // Can only be set to constant expressions
    const COUNT: u32 = 100000;
    println!("This count value is: {}", COUNT);


    // rust defaults to signed 32 bit
    // pretty cool it automatically decides the type!
    let a = 98_222; // decimal
    let b = 0xff; // hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // binary
    let e = b'A'; // Byte (u8 only)
}
