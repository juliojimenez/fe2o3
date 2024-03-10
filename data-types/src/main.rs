fn main() {
    // floating point types
    let x = 2.0; // f64
    println!("The value of x is: {x}");

    let y: f32 = 3.0; // f32
    println!("The value of y is: {y}");
    
    // numeric operations
    
    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {sum}");
    
    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {difference}");
    
    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {product}");
    
    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {quotient}");
    let trucated = -5 / 3;
    println!("The value of trucated is: {trucated}");
    
    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");
    
    // boolean type
    let t = true;
    println!("The value of t is: {t}");
    let f: bool = false; // with explicit type annotation
    println!("The value of f is: {f}");
    
    // character type
    let c = 'z';
    println!("The value of c is: {c}");
    let z = 'ℤ';
    println!("The value of z is: {z}");
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");
    
    // compound types

    // tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    let five_hundred = tup.0;
    println!("The value of five_hundred is: {five_hundred}");
    let six_point_four = tup.1;
    println!("The value of six_point_four is: {six_point_four}");
    let one = tup.2;
    println!("The value of one is: {one}");
    
    // array type
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("The value of first is: {first}");
    let second = a[1];
    println!("The value of second is: {second}");
    let _b: [i32; 5] = [1, 2, 3, 4, 5];
    let c: [i32; 5] = [3; 5];
    let first = c[0];
    println!("The value of first is: {first}");
    let second = c[1];
    println!("The value of second is: {second}");
    let third = c[2];
    println!("The value of third is: {third}");
    let fourth = c[3];
    println!("The value of fourth is: {fourth}");
    let fifth = c[4];
    println!("The value of fifth is: {fifth}");
}
