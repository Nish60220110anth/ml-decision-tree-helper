use helper_helper;

fn main() {
    println!("Simple Rust Program to help in ML Assignment-3");

    let coll_types = helper_helper::get_input_int(String::from("Enter Collection Types Count"));
    let total_count = helper_helper::get_input_int(String::from("Total Count in Collection"));
    let mut result: f32 = 0.0;

    for iter in 0..coll_types {
        println!("Iteration {}", iter);
        let pos_count = helper_helper::get_input_int(String::from("Positive Count"));
        let neg_count = helper_helper::get_input_int(String::from("Negative Count"));

        let prob = ((pos_count + neg_count) as f32) / total_count as f32;
        result += prob * helper_helper::cal_entropy_gini(pos_count, neg_count);
    }

    println!("Result {}", result);
}
