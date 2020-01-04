fn main() {
    // let x = true;
    // let y: bool = false;

    // let x = 'x';
    // let two_hearts = "♥";

    // let x = 42;
    // let y = 1.0;

    // let a = [1, 2, 3];
    // let mut m = [1, 2, 3];

    // let a = [0; 20];

    // let names = ["Graydon", "Brian", "Niko"];
    // println!("The second name is: {}", names[i]);

    // let a = [0, 1, 2, 3, 4];
    // let complete = &a[..];
    // let middle = &a[1..4];

    // let x = (1, "hello");
    // let x: (i32, &str) = (1, "hello");

    // let mut x = (1, 2);
    // let y = (2, 3);

    // x = y;

    // let (x, y, z) = (1, 2, 3);
    // println!("x is {}", x);

    // let tuple = (1, 2, 3);

    // let x = tuple.0;
    // let y = tuple.1;
    // let z = tuple.2;

    // println!("x is {}", x);

    fn foo(x: i32) -> i32 { x }
    let x: fn(i32) -> i32 = foo;
}