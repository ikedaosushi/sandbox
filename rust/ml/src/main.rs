extern crate csv;
extern crate rusty_machine;

use std::fs::File;

use rusty_machine::learning::lin_reg::LinRegressor;
use rusty_machine::learning::SupModel;
use rusty_machine::linalg::Matrix;
use rusty_machine::prelude::BaseMatrix;
use rusty_machine::linalg::Vector;

fn read_csv(filename: &str) -> (Vec<f64>, usize) {
    let mut vec: Vec<f64> = Vec::new();
    let mut line = 0;

    let file = File::open(filename).unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result.unwrap();
        line = line + 1;
        for item in record.iter() {
            vec.push(item.parse().unwrap());
        }
    }
    (vec, line)
}

fn train_test_split(data: Matrix<f64>, ratio: f64) -> (Matrix<f64>, Vector<f64>, Matrix<f64>, Vector<f64>) {
    let boundary = (data.rows() as f64 * ratio).ceil() as usize;
    let train_iter: Vec<usize> = (0..boundary).collect();
    let test_iter: Vec<usize> = (boundary+1..data.rows()).collect();
    let train = data.select_rows(train_iter.iter());
    let test = data.select_rows(test_iter.iter());

    let x_iter: Vec<usize> = (0..(data.cols()-2) as usize).collect();
    let x_train = train.select_cols(x_iter.iter());
    let tmp_y_train: Vec<f64> = train.select_cols([data.cols()-1].iter()).iter().map(|v| v.clone()).collect();
    let y_train = Vector::new(tmp_y_train);

    let x_test = test.select_cols(x_iter.iter());
    let tmp_y_test: Vec<f64> = test.select_cols([data.cols()-1].iter()).iter().map(|v| v.clone()).collect();
    let y_test = Vector::new(tmp_y_test);

    (x_train, y_train, x_test, y_test)
}

fn regression() {
    let filename = "data/house.csv";
    let (data, nrow) = read_csv(filename);
    let ncol = data.len() / nrow;
    let data = Matrix::new(nrow, ncol, data);

    // Split train/test
    let ratio = 0.99;
    let (x_train, y_train, x_test, y_test) = train_test_split(data, ratio);

    // Initiate model
    let mut reg = LinRegressor::default();

    // Train
    reg.train(&x_train, &y_train).unwrap();

    // Predict
    let y_pred = reg.predict(&x_test).unwrap();

    // Print
    for (v_pred, v_test) in y_pred.iter().zip(y_test.iter()) {
        println!("pred: {:.1}, true: {}", v_pred, v_test);
    }
}

fn main() {
    regression();
}
