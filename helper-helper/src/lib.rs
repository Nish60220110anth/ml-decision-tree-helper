use libm;
use std::io;

pub fn cal_entropy(pos_count: i32, neg_count: i32) -> f32 {
    if pos_count == neg_count {
        1.0
    } else if pos_count == 0 || neg_count == 0 {
        0.0
    } else {
        let pos_frac: f32 = ((pos_count + neg_count) as f32 / pos_count as f32) as f32;
        let neg_frac: f32 = ((pos_count + neg_count) as f32 / neg_count as f32) as f32;

        (1 as f32 / pos_frac) * libm::log2f(pos_frac)
            + (1 as f32 / neg_frac) * libm::log2f(neg_frac)
    }
}

pub fn cal_entropy_gini(pos_count: i32, neg_count: i32) -> f32 {
    let pos_frac = (pos_count as f32) / ((pos_count + neg_count) as f32);
    let neg_frac = (neg_count as f32) / ((pos_count + neg_count) as f32);

    1.0 - libm::powf(pos_frac, 2.0) - libm::powf(neg_frac, 2.0)
}

// Below two functions can be made generic but i want error handling to be
// type specific

pub fn get_input_int(query: String) -> i32 {
    println!("{}", query);
    let mut input_str = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .expect("Oops! unable to read input");
    String::from(input_str.trim())
        .parse::<i32>()
        .expect("Oops! unable to convert str to i32")
}

pub fn get_input_float(query: String) -> f32 {
    println!("{}", query);
    let mut input_str = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .expect("Oops! unable to read input");
    String::from(input_str.trim())
        .parse::<f32>()
        .expect("Oops! unable to convert str to f32")
}
