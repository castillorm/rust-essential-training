fn main() {
    // let mut x = 10;
    // println!("The value of x is: {}", x);
    // x = 20;
    // println!("The value of x is: {}", x);

    // let mut x: u8 = 255;
    // x = x + 1;
    // println!("The value of x is: {}", x);

    // let x = 3.14159265358979323846264338327950288419716939937510582097494459;
    // println!("The value of x is: {}", x);

    // let x: f32 = 3.14159265358979323846264338327950288419716939937510582097494459;
    // println!("The value of x is: {}", x);

    // let a = 10.0;
    // let b = 3.0;
    // let c = a - b;
    // let c = a / b;
    // let c = a as f64 / b;
    // let c = a as f64 / (b+1.0);
    // println!("The value of c is: {}", c);

    // Formatting print statements
    // let c = a / b;
    // // println!("The value of c is: {:08.3}\na is {}", c, a);
    // println!("The value of c is: {0:0>8.3}\na is {1}", c, a); // Using positional arguments

    // Bitwise operations
    let value = 0b1111_0101u8;
    println!("The value is: {}", value);
    println!("value is {:08b}", value);

    let not_value = !value;
    // println!("The NOT value is: {}", not_value);
    println!("NOT value is {:08b}", not_value);

    let value = value & 0b1111_0111;
    println!("AND value is {:08b}", value);
    println!("bit 6 is {}", value & 0b0100_0000);

    let value = value | 0b0100_0000;
    println!("OR value is {:08b}", value);

    let value = value ^ 0b0101_0101;
    println!("XOR value is {:08b}", value);

    let value = value << 4;
    println!("Left Shift value is {:08b}", value);

    let value = value >> 2;
    println!("Right Shift value is {:08b}", value);

}
