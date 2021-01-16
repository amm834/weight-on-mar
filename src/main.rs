use std::io;
fn main() {
    // Declare as empty string
    let mut input = String::new();
    println!("Input your weight: ");
    // Get user input from standard input
    //unwrap will terminate when program is crush
    io::stdin().read_line(&mut input).unwrap();
    //trim the white spce of user input and change to float data type by parse()
    let weight: f32 = input.trim().parse().unwrap();
    let mut weight_on_mar = calculate_weight_on_mar(weight);
    // print the output from calculate_weight_on_mar()
    println!("Weight on mar is: {}kg.", weight_on_mar);
}

fn calculate_weight_on_mar(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
