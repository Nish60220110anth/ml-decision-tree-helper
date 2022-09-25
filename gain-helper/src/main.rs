use helper_helper;
fn main() {
    println!("Gain Helper");
    let gain_mem = helper_helper::get_input_int(String::from("Enter the Column count?"));
    let total_entropy = helper_helper::get_input_float(String::from("Enter Total Entropy?"));

    for value in 1..=gain_mem {
        println!("Iteration {}", value);
        println!(
            "Value {}",
            (total_entropy - helper_helper::get_input_float(String::from("Column {} Value ?")))
        );
    }
}
