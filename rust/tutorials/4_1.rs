fn main() {
    // let x: i32;

    // println!("The value of x is: {}", x);
    let x: i32 = 17;
    let y: i32 = 18;
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 

    let z: i32 = 8;
    {
        println!("{}", z); // "8"を印字する
        let z = 12;
        println!("{}", z); // "12"を印字する
    }
    println!("{}", z); // "8"を印字する
    let z =  42;
    println!("{}", z); // "42"を印字する
}