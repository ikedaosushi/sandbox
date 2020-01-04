
fn main() {
    print_number(34);
    print_sum(32, 35);
    add_one(45);

    // without type inference
    let f: fn(i32) -> i32 = plus_one;

    let six = f(5);
    print_number(six);
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x+y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn plus_one(i: i32) -> i32 {
    i + 1
}
