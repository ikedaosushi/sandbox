fn fizzbuzz(n: usize) {
    // `for 変数 in イテレータ {…}`で繰り返しができる。
    // 指定回数の繰り返しなら`m..n`のレンジリテラルが便利。m, m+1, …, (n-1)で繰り返す。
    for i in 0..n {
        // `if 条件式 { then式 } else { else式 }`で条件分岐できる。条件の括弧は不要。
        // 条件式にはbool型しか書けないので注意。
        if i % 15 == 0 {
            println!("FizzBuzz");
        // else if はこう書く。
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            // `println!`は文字列に`{}`を使うことでフォーマッティングできる。
            println!("{}", i);
        }
    }
}

fn main() {
    fizzbuzz(20);
}