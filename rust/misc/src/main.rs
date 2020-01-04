fn fun_test(value: i32, f: &Fn(i32) -> i32) -> i32 {
    println!("{}", f(value));
    value
}

fn times2(value: i32) -> i32 {
    2 * value
}

fn main() {
    fun_test(5, &times2);
}

// extern crate gnuplot;
// use gnuplot::{Figure, Caption, Color};

// fn main() {
//     let x = [0u32, 1, 2];
//     let y = [3u32, 4, 5];
//     let mut fg = Figure::new();
//     fg.axes2d()
//     .lines(&x, &y, &[Caption("A line"), Color("black")]);
//     fg.set_terminal("png", "test.png");
//     fg.show();
// }

// extern crate gnuplot;
// extern crate nalgebra;

// use std::f64;
// use std::vec::Vec;

// use gnuplot::{Figure, Color};

// fn main() {
//     // 平均 0, 分散 1 としてインスタンス化する.
//     let mut norm = NormalDistribution::new(0.0_f64, 1.0_f64);
//     let mut x: Vec<f64> = vec![];
//     let mut n = -3.0_f64;

//     // ここで "i を 使ってないけどいいの？" と warning が出る.
//     // -3.0 から 3.0 の 0.1 刻みのベクトルを作りたかったのだがどうしたものか.
//     // この書き方はすごいセンスがない気がする.
//     for i in 0..60{
//         x.push(n);
//         n += 0.1;
//     }

//     // 一応使っておく.
//     norm.show_info();

//     // グラフの描画.
//     norm.plot(&x);
// }

// struct NormalDistribution {
//     mu : f64,
//     sigma : f64,
// }

// impl NormalDistribution {
//     fn new(mu: f64, sigma: f64) -> NormalDistribution {
//         NormalDistribution{mu: mu, sigma: sigma,}
//     }

//     // 与えられた x(ベクトル) に対して, 正規分布に従った y(ベクトル) を返す関数.
//     fn calc(&mut self, x: &Vec<f64>) -> Vec<f64> {
//         let mut y: Vec<f64> = vec![];
//         for e in x{
//             let a = - (e - self.mu).powi(2) / (2.0_f64 * self.sigma.powi(2));
//             let b = (2.0_f64 * f64::consts::PI).sqrt() * self.sigma;
//             y.push( a.exp() / b );
//         }

//         y
//     }

//     // 平均と分散をみる関数.
//     // 書き始めに練習として書いたものなので, 実際いらない.
//     fn show_info(&mut self){
//         println!("------- Information -------");
//         println!("Average: {}, Variance: {}", self.mu, self.sigma);
//         println!("---------------------------");
//     }

//     // 一番勉強したかった, メインのplot関数.
//     fn plot(&mut self, x: &Vec<f64>) {
//         let mut fg = Figure::new();
//         let y: Vec<f64> = self.calc(&x);

//         fg.axes2d().lines(x, &y, &[Color("blue")]); //xはポインタで受けてるので&がない.

//         // "png" 設定で画像を保存する.
//         fg.set_terminal("png", "NormalDistribution.png");
//         fg.show();
//     }
// }