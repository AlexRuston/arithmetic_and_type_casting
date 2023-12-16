use std::io;

fn main() {
    // arithmetic

    let x: u8 = 12;          // 0 -> 255
    let y: i8 = 10;         // -128 -> 127

    // the below calculation will not run because x and y are different types (you can't add a u to an i or f8 to f16 for example)
    /* 
        let z = x + y;
        println!("{}", z);
     */ 

    // below will print 25 because types are the same
    let a: u8 = 10;
    let v: u8 = 15; 
    
    let z = a + v;
    println!("{}", z);
    
    // % for mod operator
    // gives us the remainder after division
    // below will print 5
    // 255 goes into 10, 5 times, with 5 left over
    let f = 255;
    let g = 10;

    println!("{}", f % g);


    // type casting

    // multiple ways to assign type to a variable
    let p = 127i8;
    let q = 107_i8;
    let r: i8 = 117;
    let w = 16 as i64;
    
    println!("{} -> {} -> {} -> {}", p, q, r, w);

    // rust does not automatically convert types
    // here we've set j to i32
    let i = 160_000 as i64;
    let j: i32 = 4;

    // and here we've decided we want to make it an i64
    // we don't actually need the brackets but it is a touch more readable
    let k = i / (j as i64);

    println!("{}", k);


    // convert string input to number
    // take the users input as a string
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("expected to read line");

    println!("{}", input);

    // convert it to an int 
    let int_input: i64 = input.trim().parse().unwrap();

    // do some maths
    println!("{}", int_input + 2);
}
