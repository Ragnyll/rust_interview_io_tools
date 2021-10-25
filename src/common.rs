/// Boiler plate for i/o handling between problems
use std::io::stdin;

/// takes n lines of input (specified by first input line) then returns a vec with each line of
/// input
pub fn take_n_lines_of_input() -> Vec<String> {
    // first input will be the number of inputs to expect
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read the number of lines of input");

    let num_lines_inputs = input.trim().parse::<u16>().expect("Unable to determine number of inputs");
    // Reserve a vector of the appropriate size
    let mut sized_inputs = Vec::with_capacity(num_lines_inputs.into());

    println!("num of inputs to take {}", sized_inputs.capacity());

    // collect the number of inputs
    for _ in 0..sized_inputs.capacity() {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        for i in input.trim().split(" ") {
            sized_inputs.push(String::from(i));
        }
    }

    sized_inputs
}

/// takes n inputs delimitted by the a space char
pub fn take_n_inputs_delimited_by_space() -> Vec<String> {
    // first input will be the number of inputs to expect
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read number of inputs");

    let num_inputs = input.trim().parse::<u16>().expect("Unable to determine number of inputs");
    // Reserve a vector of the appropriate size
    let mut sized_inputs = Vec::with_capacity(num_inputs.into());

    // second input will be inputs
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    for i in input.trim().split(" ") {
        sized_inputs.push(String::from(i));
    }

    sized_inputs
}

/// given a vector of strings returns the same vector parsed for the u16 type
pub fn convert_vec_strings_to_u16(ins: Vec<String>) -> Vec<u16> {
    ins.iter()
        .map(|v: &String| -> u16 {
            v.parse::<u16>()
                .expect(&format!("Unable to convert {} to u16", v))
        })
        .collect()
}
